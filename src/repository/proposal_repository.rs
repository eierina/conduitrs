use std::error::Error;
use diesel::prelude::*;
use uuid::Uuid;

use crate::model::model::Proposal;
use crate::schema::schema::proposals;

// pub fn _create_proposal(conn: &mut PgConnection, new_proposal: &Proposal) -> QueryResult<Proposal> {
//     diesel::insert_into(proposals::table)
//         .values(new_proposal)
//         .get_result(conn)
// }

pub fn _create_proposal(conn: &mut PgConnection, new_proposal: &Proposal) -> Result<Proposal, Box<dyn Error>> {
    diesel::insert_into(proposals::table)
        .values(new_proposal)
        .get_result(conn)
        .map_err(|e| e.into()) // Converts the Diesel error into a Box<dyn Error>
}

pub fn _find_proposal_by_id(conn: &mut PgConnection, proposal_id: Uuid) -> QueryResult<Proposal> {
    proposals::table.find(proposal_id).first(conn)
}
