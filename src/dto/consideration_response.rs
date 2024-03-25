use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsiderationRequest {
    #[serde(rename = "proposalId")]
    consideration_id: Uuid,
    #[serde(rename = "payloadHint")]
    payload_hint: String,
    info: Value,
}
