use serde_json::Value;

use crate::error::OsintError;
use crate::http::Http;
use crate::source::Source;
use crate::util::digits;

pub struct BrasilApi;

impl Source for BrasilApi {
    fn name(&self) -> &'static str {
        "bank-brasilapi"
    }
    fn category(&self) -> &'static str {
        "financial"
    }
    fn kind(&self) -> &'static str {
        "bank"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("001")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let code = digits(query);
        if code.is_empty() {
            return Err(OsintError::Invalid("bank code expected".into()));
        }
        http.get_json(&format!("https://brasilapi.com.br/api/banks/v1/{code}"))
    }
}
