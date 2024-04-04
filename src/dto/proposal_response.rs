use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalResponse {
    #[serde(rename = "proposalId")]
    pub proposal_id: Uuid,
    pub participants: Vec<Uuid>,
    #[serde(rename = "payloadHint")]
    pub payload_hint: String,
    pub info: Value,
}
