use actix_web::{get, HttpResponse, post, Responder, web};
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use crate::dto::proposal_request::ProposalRequest;
use crate::model::model::Proposal;
use crate::repository::proposal_repository;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/proposals/{id}")]
async fn get_proposal(path: web::Path<(u32,)>) -> impl Responder {
    let id = path.into_inner().0;
    HttpResponse::Ok().body(format!("Retrieving proposal with ID: {}", id))
}

#[post("/proposals")]
async fn create_proposal(
    pool: web::Data<DbPool>,
    proposal_json: web::Json<ProposalRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let proposal_request: ProposalRequest = proposal_json.into_inner().into();
    let proposal: Proposal = proposal_request.to_entity();

    match proposal_repository::_create_proposal(&mut conn, &proposal) {
        Ok(proposal) => HttpResponse::Created().json(proposal.to_response()),
        Err(e) => {
            println!("Error: {}", e);
            HttpResponse::InternalServerError().into()
        },
    }
}
