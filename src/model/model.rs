use diesel::associations::*;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::schema::{agreements, considerations, proposals};

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(table_name = proposals)]
pub struct Proposal {
    pub id: Uuid,
    pub maker_id: Uuid,
    pub participants: Vec<Uuid>,
    pub payload_hint: String,
    pub info: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Insertable, Debug, PartialEq)]
#[diesel(belongs_to(Proposal))]
#[diesel(table_name = considerations)]
pub struct Consideration {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub taker_id: Uuid,
    pub payload_hint: String,
    pub info: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Insertable, Debug, PartialEq)]
#[diesel(belongs_to(Consideration))]
#[diesel(table_name = agreements)]
pub struct Agreement {
    pub id: Uuid,
    pub consideration_id: Uuid,
    pub maker_id: Uuid,
    pub taker_id: Uuid,
    pub payload_hint: String,
    pub info: String,
}
