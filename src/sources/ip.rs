use std::net::IpAddr;

use serde_json::Value;

use crate::error::OsintError;
use crate::http::Http;
use crate::source::Source;

fn addr(query: &str) -> Result<IpAddr, OsintError> {
    query
        .trim()
        .parse::<IpAddr>()
        .map_err(|_| OsintError::Invalid("not a valid ip address".into()))
}

pub struct IpApi;
pub struct InternetDb;
pub struct Rdap;

impl Source for IpApi {
    fn name(&self) -> &'static str {
        "ip-ipapi"
    }
    fn category(&self) -> &'static str {
        "geo"
    }
    fn kind(&self) -> &'static str {
        "ip"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("8.8.8.8")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let ip = addr(query)?;
        let body = http.get_json(&format!("http://ip-api.com/json/{ip}"))?;
        if body.get("status").and_then(|v| v.as_str()) == Some("fail") {
            return Err(OsintError::NotFound);
        }
        Ok(body)
    }
}

impl Source for InternetDb {
    fn name(&self) -> &'static str {
        "ip-internetdb"
    }
    fn category(&self) -> &'static str {
        "infrastructure"
    }
    fn kind(&self) -> &'static str {
        "ip"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("8.8.8.8")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let ip = addr(query)?;
        http.get_json(&format!("https://internetdb.shodan.io/{ip}"))
    }
}

impl Source for Rdap {
    fn name(&self) -> &'static str {
        "ip-rdap"
    }
    fn category(&self) -> &'static str {
        "infrastructure"
    }
    fn kind(&self) -> &'static str {
        "ip"
    }
    fn probe(&self) -> Option<&'static str> {
        Some("8.8.8.8")
    }
    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError> {
        let ip = addr(query)?;
        http.get_json(&format!("https://rdap.org/ip/{ip}"))
    }
}
