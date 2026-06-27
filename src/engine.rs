use std::time::Instant;

use crate::error::OsintError;
use crate::http::Http;
use crate::model::{Outcome, Status};
use crate::source::Source;

pub fn detect(query: &str) -> Option<&'static str> {
    let trimmed = query.trim();
    if trimmed.parse::<std::net::IpAddr>().is_ok() {
        return Some("ip");
    }

    let stripped: String = trimmed
        .chars()
        .filter(|c| !matches!(c, '.' | '-' | '/' | ' '))
        .collect();
    if !stripped.is_empty() && stripped.chars().all(|c| c.is_ascii_digit()) {
        return match stripped.len() {
            14 => Some("cnpj"),
            11 => Some("cpf"),
            8 => Some("cep"),
            3 => Some("bank"),
            2 => Some("ddd"),
            _ => None,
        };
    }
    if trimmed.contains('.') {
        return Some("domain");
    }
    None
}

pub fn collect(kind: &str, query: &str, http: &Http, sources: &[Box<dyn Source>]) -> Vec<Outcome> {
    let picked: Vec<&dyn Source> = sources
        .iter()
        .map(|b| b.as_ref())
        .filter(|s| s.kind() == kind)
        .collect();

    let mut results: Vec<Outcome> = std::thread::scope(|scope| {
        let handles: Vec<_> = picked
            .into_iter()
            .map(|source| scope.spawn(move || run_one(source, query, http)))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    results.sort_by(|a, b| a.source.cmp(&b.source));
    results
}

fn run_one(source: &dyn Source, query: &str, http: &Http) -> Outcome {
    let started = Instant::now();
    let answer = source.fetch(query, http);
    let elapsed_ms = started.elapsed().as_millis();

    let (status, data, error) = match answer {
        Ok(value) => (Status::Ok, Some(value), None),
        Err(OsintError::NotFound) => (Status::NotFound, None, None),
        Err(other) => (Status::Error, None, Some(other.to_string())),
    };

    Outcome {
        source: source.name().to_string(),
        category: source.category().to_string(),
        query: query.to_string(),
        status,
        data,
        error,
        elapsed_ms,
    }
}
