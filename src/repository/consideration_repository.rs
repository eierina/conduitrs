use diesel::prelude::*;
use uuid::Uuid;

use crate::model::model::Consideration;
use crate::schema::schema::considerations::dsl::*;

/// Inserts a new consideration into the database.
pub fn insert_consideration(conn: &mut PgConnection, new_consideration: &Consideration) -> Result<Consideration, diesel::result::Error> {
    diesel::insert_into(considerations)
        .values(new_consideration)
        .get_result(conn)
}

/// Retrieves a consideration by its ID.
pub fn find_consideration_by_id(conn: &mut PgConnection, cons_id: Uuid) -> Result<Consideration, diesel::result::Error> {
    considerations
        .find(cons_id)
        .first(conn)
}

/// Retrieves all considerations associated with a given proposal ID.
pub fn find_considerations_by_proposal_id(conn: &mut PgConnection, proposal_id_val: Uuid) -> Result<Vec<Consideration>, diesel::result::Error> {
    considerations
        .filter(proposal_id.eq(proposal_id_val))
        .load(conn)
}

/// Updates an existing consideration with new data.
// pub fn update_consideration(conn: &mut PgConnection, cons_id: Uuid, cons_changes: &Consideration) -> Result<Consideration, diesel::result::Error> {
//     diesel::update(considerations.find(cons_id))
//         .set(cons_changes)
//         .get_result(conn)
// }

/// Deletes a consideration from the database.
pub fn delete_consideration(conn: &mut PgConnection, cons_id: Uuid) -> Result<usize, diesel::result::Error> {
    diesel::delete(considerations.find(cons_id))
        .execute(conn)
}

/// Lists all considerations in the database.
pub fn list_considerations(conn: &mut PgConnection) -> Result<Vec<Consideration>, diesel::result::Error> {
    considerations.load(conn)
}
