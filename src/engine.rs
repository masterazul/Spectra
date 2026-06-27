use std::sync::Mutex;
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
            2 | 3 => Some("ddd"),
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

    let bag: Mutex<Vec<Outcome>> = Mutex::new(Vec::with_capacity(picked.len()));
    std::thread::scope(|scope| {
        for source in picked {
            let bag = &bag;
            scope.spawn(move || {
                let outcome = run_one(source, query, http);
                bag.lock().unwrap().push(outcome);
            });
        }
    });

    let mut results = bag.into_inner().unwrap();
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
