use serde_json::Value;

use crate::error::OsintError;
use crate::http::Http;
use crate::source::Source;
use crate::util::digits;

fn normalize(query: &str) -> Result<String, OsintError> {
    let d = digits(query);
    if d.len() != 14 {
        return Err(OsintError::Invalid("cnpj must contain 14 digits".into()));
    }
    Ok(d)
}

pub struct BrasilApi;
pub struct MinhaReceita;

impl Source for BrasilApi {
    fn name(&self) -> &'static str {
        "cnpj-brasilapi"
    }
    fn category(&self) -> &'static str {
        "corporate"
    }
    fn kind(&self) -> &'static str {
        "cnpj"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("00000000000191")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let cnpj = normalize(query)?;
        http.get_json(&format!("https://brasilapi.com.br/api/cnpj/v1/{cnpj}"))
    }
}

impl Source for MinhaReceita {
    fn name(&self) -> &'static str {
        "cnpj-minhareceita"
    }
    fn category(&self) -> &'static str {
        "corporate"
    }
    fn kind(&self) -> &'static str {
        "cnpj"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("00000000000191")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let cnpj = normalize(query)?;
        http.get_json(&format!("https://minhareceita.org/{cnpj}"))
    }
}
