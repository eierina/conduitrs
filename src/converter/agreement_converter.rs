
use std::iter;
use uuid::Uuid;

use crate::dto::agreement_request::AgreementRequest;
use crate::dto::agreement_response::AgreementResponse;
use crate::model::model::Agreement;

impl AgreementRequest {
    pub fn to_entity(&self, consideration_id: Uuid, maker_id: Uuid, taker_id: Uuid) -> Agreement {
        Agreement {
            id: self.agreement_id,
            consideration_id,
            maker_id,
            taker_id,
            payload_hint: self.payload_hint.clone(),
            info: self.payload.to_string(),
        }
    }
}

impl Agreement {
    pub fn to_response(&self) -> AgreementResponse {
        AgreementResponse {
            agreement_id: self.id,
            payload_hint: self.payload_hint.clone(),
            payload: serde_json::from_str(&self.info).unwrap(),
        }
    }
}
