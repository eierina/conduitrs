use diesel::prelude::*;
use uuid::Uuid;

use crate::model::model::Proposal;
use crate::schema::schema::proposals::dsl::proposals;

/// Inserts a new proposal into the database.
pub fn insert_proposal(conn: &mut PgConnection, new_proposal: &Proposal) -> Result<Proposal, diesel::result::Error> {
    diesel::insert_into(proposals)
        .values(new_proposal)
        .get_result(conn)
}

/// Retrieves a proposal by its ID.
pub fn find_proposal_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Proposal, diesel::result::Error> {
    proposals.find(id).first(conn)
}
