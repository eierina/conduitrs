use std::iter;

use crate::dto::proposal_request::ProposalRequest;
use crate::dto::proposal_response::ProposalResponse;
use crate::model::model::Proposal;

impl ProposalRequest {
    pub fn to_entity(&self) -> Proposal {
        Proposal {
            id: self.proposal_id,
            maker_id: self.participants[0],
            participants: self.participants.iter().skip(1).cloned().collect(),
            payload_hint: self.payload_hint.clone(),
            info: self.info.to_string(),
        }
    }
}

impl Proposal {
    pub fn to_response(&self) -> ProposalResponse {
        ProposalResponse {
            proposal_id: self.id,
            participants: iter::once(self.maker_id)
                .chain(self.participants.iter().cloned())
                .collect(),
            payload_hint: self.payload_hint.clone(),
            info: serde_json::from_str(&self.info).unwrap(),
        }
    }
}
