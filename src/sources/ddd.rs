use serde_json::Value;

use crate::error::OsintError;
use crate::http::Http;
use crate::source::Source;
use crate::util::digits;

pub struct BrasilApi;

impl Source for BrasilApi {
    fn name(&self) -> &'static str {
        "ddd-brasilapi"
    }
    fn category(&self) -> &'static str {
        "telecom"
    }
    fn kind(&self) -> &'static str {
        "ddd"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("21")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let ddd = digits(query);
        if ddd.len() != 2 {
            return Err(OsintError::Invalid("ddd must contain 2 digits".into()));
        }
        http.get_json(&format!("https://brasilapi.com.br/api/ddd/v1/{ddd}"))
    }
}
