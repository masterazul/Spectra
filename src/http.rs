use std::sync::Arc;
use std::time::Duration;

use serde_json::Value;

use crate::error::OsintError;

pub struct Http {
    agent: ureq::Agent,
}

impl Http {
    pub fn new() -> Self {
        let tls = native_tls::TlsConnector::new().expect("failed to build tls connector");
        let agent = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(12))
            .user_agent(concat!("spectra/", env!("CARGO_PKG_VERSION")))
            .tls_connector(Arc::new(tls))
            .build();
        Self { agent }
    }

    pub fn get_json(&self, url: &str) -> Result<Value, OsintError> {
        match self.agent.get(url).call() {
            Ok(resp) => resp
                .into_json::<Value>()
                .map_err(|e| OsintError::Decode(e.to_string())),
            Err(ureq::Error::Status(404, _)) => Err(OsintError::NotFound),
            Err(ureq::Error::Status(code, _)) => Err(OsintError::Status(code)),
            Err(ureq::Error::Transport(t)) => Err(OsintError::Transport(t.to_string())),
        }
    }

    pub fn head_info(&self, url: &str) -> Result<Value, OsintError> {
        let collect = |resp: ureq::Response| {
            let mut map = serde_json::Map::new();
            map.insert("status".into(), Value::from(resp.status()));
            for name in resp.headers_names() {
                if let Some(value) = resp.header(&name) {
                    map.insert(name, Value::from(value));
                }
            }
            Value::Object(map)
        };
        match self.agent.get(url).call() {
            Ok(resp) => Ok(collect(resp)),
            Err(ureq::Error::Status(_, resp)) => Ok(collect(resp)),
            Err(ureq::Error::Transport(t)) => Err(OsintError::Transport(t.to_string())),
        }
    }
}

impl Default for Http {
    fn default() -> Self {
        Self::new()
    }
}
