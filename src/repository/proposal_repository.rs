use std::error::Error;
use diesel::prelude::*;
use uuid::Uuid;

use crate::model::model::Proposal;
use crate::schema::schema::proposals::dsl::*;

pub fn create_proposal(conn: &mut PgConnection, new_proposal: &Proposal) -> Result<Proposal, Box<dyn Error>> {
    diesel::insert_into(proposals)
        .values(new_proposal)
        .get_result(conn)
        .map_err(|e| e.into())
}

pub fn find_proposal_by_id(conn: &mut PgConnection, proposal_id_param: Uuid) -> Result<Proposal, Box<dyn Error>> {
    proposals.find(proposal_id_param).first(conn).map_err(|e| e.into())
}
