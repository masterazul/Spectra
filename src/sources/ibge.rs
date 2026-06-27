use serde_json::Value;

use crate::error::OsintError;
use crate::http::Http;
use crate::source::Source;

pub struct Municipios;

impl Source for Municipios {
    fn name(&self) -> &'static str {
        "ibge-municipios"
    }
    fn category(&self) -> &'static str {
        "geo"
    }
    fn kind(&self) -> &'static str {
        "uf"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("RJ")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let uf = query.trim().to_uppercase();
        if uf.len() != 2 || !uf.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(OsintError::Invalid("expected a 2-letter state code".into()));
        }
        let url =
            format!("https://servicodados.ibge.gov.br/api/v1/localidades/estados/{uf}/municipios");
        let body = http.get_json(&url)?;
        match &body {
            Value::Array(items) if items.is_empty() => Err(OsintError::NotFound),
            _ => Ok(body),
        }
    }
}
