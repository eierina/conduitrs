
use std::iter;
use uuid::Uuid;

use crate::dto::consideration_request::ConsiderationRequest;
use crate::dto::consideration_response::ConsiderationResponse;
use crate::model::model::Consideration;

impl ConsiderationRequest {
    pub fn to_entity(&self, proposal_id: Uuid, taker_id: Uuid) -> Consideration {
        Consideration {
            id: self.consideration_id,
            proposal_id,
            taker_id,
            payload_hint: self.payload_hint.clone(),
            info: self.payload.to_string(),
        }
    }
}

impl Consideration {
    pub fn to_response(&self) -> ConsiderationResponse {
        ConsiderationResponse {
            consideration_id: self.id,
            payload_hint: self.payload_hint.clone(),
            payload: serde_json::from_str(&self.info).unwrap(),
        }
    }
}
