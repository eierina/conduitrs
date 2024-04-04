use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AgreementRequest {
    #[serde(rename = "agreementId")]
    pub agreement_id: Uuid,
    #[serde(rename = "payloadHint")]
    pub payload_hint: String,
    pub info: Value,
}
