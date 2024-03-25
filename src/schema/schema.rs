use diesel::prelude::*;
use diesel::table;

table! {
    proposals (id) {
        id -> Uuid,
        maker_id -> Uuid,
        participants -> Array<Uuid>,
        payload_hint -> Varchar,
        info -> Text,
    }
}

table! {
    considerations (id) {
        id -> Uuid,
        proposal_id -> Uuid,
        taker_id -> Uuid,
        payload_hint -> Varchar,
        info -> Text,
    }
}

table! {
    agreements (id) {
        id -> Uuid,
        consideration_id -> Uuid,
        maker_id -> Uuid,
        taker_id -> Uuid,
        payload_hint -> Varchar,
        info -> Text,
    }
}

joinable!(considerations -> proposals (proposal_id));
joinable!(agreements -> considerations (consideration_id));

allow_tables_to_appear_in_same_query!(proposals, considerations, agreements,);
