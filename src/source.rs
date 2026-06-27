use serde_json::Value;

use crate::error::OsintError;
use crate::http::Http;

pub trait Source: Send + Sync {
    fn name(&self) -> &'static str;
    fn category(&self) -> &'static str;
    fn kind(&self) -> &'static str;

    fn requires_key(&self) -> bool {
        false
    }

    fn probe(&self) -> Option<&'static str> {
        None
    }

    fn fetch(&self, query: &str, http: &Http) -> Result<Value, OsintError>;
}
