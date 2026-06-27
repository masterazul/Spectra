use serde_json::{Map, Value};

use crate::error::OsintError;
use crate::http::Http;
use crate::source::Source;

fn host(query: &str) -> Result<&str, OsintError> {
    let h = query.trim().trim_end_matches('.');
    if h.is_empty() || h.contains(' ') || !h.contains('.') {
        return Err(OsintError::Invalid("domain expected".into()));
    }
    Ok(h)
}

pub struct GoogleDns;
pub struct CrtSh;
pub struct Rdap;
pub struct Headers;

impl Source for GoogleDns {
    fn name(&self) -> &'static str {
        "domain-googledns"
    }
    fn category(&self) -> &'static str {
        "infrastructure"
    }
    fn kind(&self) -> &'static str {
        "domain"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("github.com")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let h = host(query)?;
        let mut records = Map::new();
        let mut found = false;
        for rtype in ["A", "AAAA", "MX", "NS", "TXT"] {
            let answer =
                http.get_json(&format!("https://dns.google/resolve?name={h}&type={rtype}"));
            match answer {
                Ok(value) => {
                    let entries = value.get("Answer").cloned().unwrap_or(Value::Array(vec![]));
                    if entries.as_array().map(|a| !a.is_empty()).unwrap_or(false) {
                        found = true;
                    }
                    records.insert(rtype.to_string(), entries);
                }
                Err(OsintError::NotFound) => {}
                Err(other) => return Err(other),
            }
        }
        if !found {
            return Err(OsintError::NotFound);
        }
        Ok(Value::Object(records))
    }
}

impl Source for CrtSh {
    fn name(&self) -> &'static str {
        "domain-crtsh"
    }
    fn category(&self) -> &'static str {
        "infrastructure"
    }
    fn kind(&self) -> &'static str {
        "domain"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("github.com")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let h = host(query)?;
        let body = http.get_json(&format!("https://crt.sh/?q={h}&output=json"))?;
        match &body {
            Value::Array(items) if items.is_empty() => Err(OsintError::NotFound),
            _ => Ok(body),
        }
    }
}

impl Source for Rdap {
    fn name(&self) -> &'static str {
        "domain-rdap"
    }
    fn category(&self) -> &'static str {
        "infrastructure"
    }
    fn kind(&self) -> &'static str {
        "domain"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("github.com")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let h = host(query)?;
        http.get_json(&format!("https://rdap.org/domain/{h}"))
    }
}

impl Source for Headers {
    fn name(&self) -> &'static str {
        "domain-headers"
    }
    fn category(&self) -> &'static str {
        "infrastructure"
    }
    fn kind(&self) -> &'static str {
        "domain"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("github.com")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let h = host(query)?;
        http.head_info(&format!("https://{h}"))
    }
}
