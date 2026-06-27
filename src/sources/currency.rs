use serde_json::Value;

use crate::error::OsintError;
use crate::http::Http;
use crate::source::Source;

pub struct AwesomeApi;

impl Source for AwesomeApi {
    fn name(&self) -> &'static str {
        "currency-awesomeapi"
    }
    fn category(&self) -> &'static str {
        "financial"
    }
    fn kind(&self) -> &'static str {
        "currency"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("USD-BRL")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let pair = query.trim().to_uppercase();
        if !pair.contains('-') {
            return Err(OsintError::Invalid("expected a pair like USD-BRL".into()));
        }
        http.get_json(&format!(
            "https://economia.awesomeapi.com.br/json/last/{pair}"
        ))
    }
}
