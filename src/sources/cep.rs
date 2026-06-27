use serde_json::Value;

use crate::error::OsintError;
use crate::http::Http;
use crate::source::Source;
use crate::util::digits;

pub struct ViaCep;
pub struct BrasilApi;

impl Source for ViaCep {
    fn name(&self) -> &'static str {
        "cep-viacep"
    }
    fn category(&self) -> &'static str {
        "address"
    }
    fn kind(&self) -> &'static str {
        "cep"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("01310100")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let cep = digits(query);
        if cep.len() != 8 {
            return Err(OsintError::Invalid("cep must contain 8 digits".into()));
        }
        let body = http.get_json(&format!("https://viacep.com.br/ws/{cep}/json/"))?;
        if body.get("erro").is_some() {
            return Err(OsintError::NotFound);
        }
        Ok(body)
    }
}

impl Source for BrasilApi {
    fn name(&self) -> &'static str {
        "cep-brasilapi"
    }
    fn category(&self) -> &'static str {
        "address"
    }
    fn kind(&self) -> &'static str {
        "cep"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("01310100")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let cep = digits(query);
        if cep.len() != 8 {
            return Err(OsintError::Invalid("cep must contain 8 digits".into()));
        }
        http.get_json(&format!("https://brasilapi.com.br/api/cep/v2/{cep}"))
    }
}
