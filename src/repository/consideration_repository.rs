use std::error::Error;
use diesel::prelude::*;
use uuid::Uuid;

use crate::model::model::{Consideration, Proposal};
use crate::schema::schema::considerations::dsl::*;
use crate::schema::schema::proposals::dsl::*;

pub fn create_consideration(conn: &mut PgConnection, new_consideration: &Consideration) -> Result<Consideration, Box<dyn Error>> {
    let proposal: QueryResult<Proposal> = proposals.find(new_consideration.proposal_id).first(conn);
    if proposal.is_err() {
        return Err(Box::from("Proposal not found"));
    }
    let proposal = proposal.unwrap();
    if proposal.participants.len() > 0 && !proposal.participants.contains(&new_consideration.taker_id) {
        return Err(Box::from("Cannot create consideration for this proposal"));
    }
    diesel::insert_into(considerations)
        .values(new_consideration)
        .get_result(conn)
        .map_err(|e| e.into())
}

pub fn find_consideration_by_id(conn: &mut PgConnection, consideration_id_param: Uuid) -> Result<Consideration, Box<dyn Error>> {
    considerations.find(consideration_id_param).first(conn).map_err(|e| e.into())
}

pub fn find_considerations_by_proposal_id(conn: &mut PgConnection, proposal_id_param: Uuid) -> Result<Vec<Consideration>, Box<dyn Error>>  {
    considerations
        .filter(proposal_id.eq(proposal_id_param))
        .load::<Consideration>(conn)
        .map_err(|e| e.into())
}