use std::process::ExitCode;

use spectra::model::Status;
use spectra::{engine, http::Http, sources, util, validate, verify};

const USAGE: &str = "spectra - OSINT toolkit for Brazilian public-record sources

usage:
  spectra <command> [value] [--json]

commands:
  cnpj <cnpj>      company registry
  cep <cep>        postal address
  ddd <ddd>        phone area code
  bank <code>      bank registry
  domain <name>    dns records, certificate transparency, rdap, headers
  ip <addr>        geolocation, asn, exposed services
  currency <pair>  exchange rate, e.g. USD-BRL
  uf <state>       municipalities of a state, e.g. RJ
  holidays <year>  national holidays
  isbn <code>      book metadata
  ncm <code>       customs product code
  lookup <value>   auto-detect the query type
  validate <doc>   cpf/cnpj check digits, offline
  verify           probe every source
  sources          list registered sources
  version          print version
  help             show this message
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let json = args.iter().any(|a| a == "--json");
    let mut positional = args.iter().filter(|a| !a.starts_with("--"));

    let command = match positional.next() {
        Some(c) => c.as_str(),
        None => {
            print!("{USAGE}");
            return ExitCode::FAILURE;
        }
    };
    let value = positional.next().map(String::as_str);

    match command {
        "help" | "-h" | "--help" => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        "version" | "-V" | "--version" => {
            println!("spectra {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        "sources" => {
            list_sources(json);
            ExitCode::SUCCESS
        }
        "verify" => run_verify(json),
        "validate" => match value {
            Some(doc) => run_validate(doc, json),
            None => fail("validate needs a document"),
        },
        "lookup" => match value {
            Some(v) => run_lookup(v, json),
            None => fail("lookup needs a value"),
        },
        kind @ ("cnpj" | "cep" | "ddd" | "bank" | "domain" | "ip" | "currency" | "uf"
        | "holidays" | "isbn" | "ncm") => match value {
            Some(v) => run_collect(kind, v, json),
            None => fail(&format!("{kind} needs a value")),
        },
        other => {
            eprintln!("unknown command: {other}\n");
            print!("{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn fail(message: &str) -> ExitCode {
    eprintln!("{message}");
    ExitCode::FAILURE
}

fn run_collect(kind: &str, value: &str, json: bool) -> ExitCode {
    let http = match Http::new() {
        Ok(h) => h,
        Err(e) => return fail(&e.to_string()),
    };
    let registry = sources::all();
    let results = engine::collect(kind, value, &http, &registry);
    if results.is_empty() {
        return fail(&format!("no sources registered for '{kind}'"));
    }
    emit(&results, json);
    if results.iter().any(|o| o.status == Status::Ok) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn run_lookup(value: &str, json: bool) -> ExitCode {
    match engine::detect(value) {
        Some("cpf") => run_validate(value, json),
        Some(kind) => run_collect(kind, value, json),
        None => fail(&format!("could not detect query type for '{value}'")),
    }
}

fn run_validate(doc: &str, json: bool) -> ExitCode {
    let cleaned = util::digits(doc);
    let (kind, valid) = match cleaned.len() {
        11 => ("cpf", validate::cpf(doc)),
        14 => ("cnpj", validate::cnpj(doc)),
        _ => ("unknown", false),
    };
    if json {
        let payload = serde_json::json!({ "document": cleaned, "kind": kind, "valid": valid });
        println!("{payload}");
    } else {
        let verdict = if valid { "valid" } else { "invalid" };
        println!("{kind} {cleaned}: {verdict}");
    }
    if valid {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn run_verify(json: bool) -> ExitCode {
    let http = match Http::new() {
        Ok(h) => h,
        Err(e) => return fail(&e.to_string()),
    };
    let registry = sources::all();
    let report = verify::check_all(&http, &registry);
    if json {
        emit(&report, true);
    } else {
        for h in &report {
            let state = if h.alive { "up" } else { "down" };
            let detail = h.detail.as_deref().unwrap_or("");
            println!(
                "[{state:>4}] {:<20} {:<14} {:>5}ms  {detail}",
                h.source, h.kind, h.latency_ms
            );
        }
    }
    if report.iter().all(|h| h.alive) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn list_sources(json: bool) {
    let registry = sources::all();
    if json {
        let items: Vec<_> = registry
            .iter()
            .map(|s| {
                serde_json::json!({
                    "name": s.name(),
                    "kind": s.kind(),
                    "category": s.category(),
                    "requires_key": s.requires_key(),
                })
            })
            .collect();
        emit(&items, false);
    } else {
        for s in &registry {
            println!("{:<20} {:<10} {}", s.name(), s.kind(), s.category());
        }
    }
}

fn emit<T: serde::Serialize>(value: &T, compact: bool) {
    let rendered = if compact {
        serde_json::to_string(value)
    } else {
        serde_json::to_string_pretty(value)
    };
    println!("{}", rendered.unwrap_or_else(|_| "null".to_string()));
}
