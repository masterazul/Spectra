use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ok,
    NotFound,
    Error,
}

#[derive(Serialize)]
pub struct Outcome {
    pub source: String,
    pub kind: String,
    pub category: String,
    pub query: String,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub elapsed_ms: u128,
}
