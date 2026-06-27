use serde_json::Value;

use crate::error::OsintError;
use crate::http::Http;
use crate::source::Source;

fn valid_pair(pair: &str) -> bool {
    let mut parts = pair.split('-');
    matches!((parts.next(), parts.next(), parts.next()), (Some(a), Some(b), None)
        if a.len() == 3 && b.len() == 3
        && a.bytes().all(|c| c.is_ascii_alphabetic())
        && b.bytes().all(|c| c.is_ascii_alphabetic()))
}

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
        if !valid_pair(&pair) {
            return Err(OsintError::Invalid("expected a pair like USD-BRL".into()));
        }
        http.get_json(&format!(
            "https://economia.awesomeapi.com.br/json/last/{pair}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::valid_pair;

    #[test]
    fn pair_shape() {
        assert!(valid_pair("USD-BRL"));
        assert!(!valid_pair("USD-BRL/x"));
        assert!(!valid_pair("US-BRL"));
        assert!(!valid_pair("USDBRL"));
        assert!(!valid_pair("USD-BR2"));
    }
}
