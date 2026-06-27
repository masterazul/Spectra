use serde_json::Value;

use crate::error::OsintError;
use crate::http::Http;
use crate::source::Source;
use crate::util::digits;

pub struct Holidays;
pub struct Isbn;
pub struct Ncm;

impl Source for Holidays {
    fn name(&self) -> &'static str {
        "holidays-brasilapi"
    }
    fn category(&self) -> &'static str {
        "reference"
    }
    fn kind(&self) -> &'static str {
        "holidays"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("2025")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let year = digits(query);
        if year.len() != 4 {
            return Err(OsintError::Invalid("year must have 4 digits".into()));
        }
        http.get_json(&format!("https://brasilapi.com.br/api/feriados/v1/{year}"))
    }
}

impl Source for Isbn {
    fn name(&self) -> &'static str {
        "isbn-brasilapi"
    }
    fn category(&self) -> &'static str {
        "reference"
    }
    fn kind(&self) -> &'static str {
        "isbn"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("9788535902778")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let code: String = query
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect();
        if code.len() < 10 {
            return Err(OsintError::Invalid("isbn expected".into()));
        }
        http.get_json(&format!("https://brasilapi.com.br/api/isbn/v1/{code}"))
    }
}

impl Source for Ncm {
    fn name(&self) -> &'static str {
        "ncm-brasilapi"
    }
    fn category(&self) -> &'static str {
        "reference"
    }
    fn kind(&self) -> &'static str {
        "ncm"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("33030010")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let code = digits(query);
        if code.is_empty() {
            return Err(OsintError::Invalid("ncm code expected".into()));
        }
        http.get_json(&format!("https://brasilapi.com.br/api/ncm/v1/{code}"))
    }
}
