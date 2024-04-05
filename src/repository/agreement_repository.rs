use diesel::prelude::*;
use uuid::Uuid;

use crate::model::model::Agreement;
use crate::schema::schema::agreements::dsl::*;

/// Inserts a new agreement into the database.
pub fn insert_agreement(conn: &mut PgConnection, new_agreement: &Agreement) -> Result<Agreement, diesel::result::Error> {
    diesel::insert_into(agreements)
        .values(new_agreement)
        .get_result(conn)
}

/// Retrieves an agreement by its ID.
pub fn find_agreement_by_id(conn: &mut PgConnection, agreement_id_val: Uuid) -> Result<Agreement, diesel::result::Error> {
    agreements
        .find(agreement_id_val)
        .first(conn)
}

/// Retrieves all agreements associated with a given consideration ID.
pub fn find_agreements_by_consideration_id(conn: &mut PgConnection, consideration_id_val: Uuid) -> Result<Vec<Agreement>, diesel::result::Error> {
    agreements
        .filter(consideration_id.eq(consideration_id_val))
        .load(conn)
}

/// Finds agreements associated with a given proposal ID by joining the `agreements`
/// and `considerations` tables and filtering by the `proposal_id`.
pub fn find_agreements_by_proposal_id(conn: &mut PgConnection, proposal_id_val: Uuid) -> Result<Vec<Agreement>, diesel::result::Error> {
    use crate::schema::schema::considerations::dsl::{considerations, id as considerations_id, proposal_id as consideration_proposal_id};
    use crate::schema::schema::agreements::dsl::*;

    agreements
        .inner_join(considerations.on(consideration_id.eq(considerations_id)))
        .filter(consideration_proposal_id.eq(proposal_id_val))
        .select((id, consideration_id, maker_id, taker_id, payload_hint, info)) // Ensure you select the desired columns or use all_columns if applicable
        .load(conn)
}

/// Updates an existing agreement with new data.
// pub fn update_agreement(conn: &mut PgConnection, agreement_id_val: Uuid, agreement_changes: &Agreement) -> Result<Agreement, diesel::result::Error> {
//     diesel::update(agreements.find(agreement_id_val))
//         .set(agreement_changes)
//         .get_result(conn)
// }

/// Deletes an agreement from the database.
pub fn delete_agreement(conn: &mut PgConnection, agreement_id_val: Uuid) -> Result<usize, diesel::result::Error> {
    diesel::delete(agreements.find(agreement_id_val))
        .execute(conn)
}

/// Lists all agreements in the database.
pub fn list_agreements(conn: &mut PgConnection) -> Result<Vec<Agreement>, diesel::result::Error> {
    agreements.load(conn)
}
