use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsiderationResponse {
    #[serde(rename = "considerationId")]
    pub consideration_id: Uuid,
    #[serde(rename = "payloadHint")]
    pub payload_hint: String,
    pub info: Value,
}
