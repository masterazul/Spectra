use std::time::Instant;

use serde::Serialize;

use crate::error::OsintError;
use crate::http::Http;
use crate::source::Source;

#[derive(Serialize)]
pub struct Health {
    pub source: String,
    pub category: String,
    pub kind: String,
    pub alive: bool,
    pub latency_ms: u128,
    pub requires_key: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

pub fn check_all(http: &Http, sources: &[Box<dyn Source>]) -> Vec<Health> {
    let mut report: Vec<Health> = std::thread::scope(|scope| {
        let handles: Vec<_> = sources
            .iter()
            .map(|b| b.as_ref())
            .map(|source| scope.spawn(move || probe(source, http)))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    report.sort_by(|a, b| a.source.cmp(&b.source));
    report
}

fn probe(source: &dyn Source, http: &Http) -> Health {
    let started = Instant::now();
    let (alive, detail) = match source.probe() {
        None => (false, Some("no probe configured".to_string())),
        Some(sample) => match source.fetch(sample, http) {
            Ok(_) => (true, None),
            Err(OsintError::NotFound) => (true, Some("reachable, sample empty".to_string())),
            Err(err) => (false, Some(err.to_string())),
        },
    };
    Health {
        source: source.name().to_string(),
        category: source.category().to_string(),
        kind: source.kind().to_string(),
        alive,
        latency_ms: started.elapsed().as_millis(),
        requires_key: source.requires_key(),
        detail,
    }
}
