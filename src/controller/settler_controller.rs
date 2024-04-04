use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web, Error, ResponseError};
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::Deserialize;
use uuid::Uuid;
use crate::dto::error_response::ErrorResponse;
use crate::dto::proposal_request::ProposalRequest;
use crate::dto::consideration_request::ConsiderationRequest;
use crate::dto::consideration_response::ConsiderationResponse;
use crate::repository::proposal_repository;
use crate::repository::consideration_repository;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/proposals/{id}")]
async fn get_proposal(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, impl ResponseError> {
    let id = path.into_inner();
    let mut conn = pool.get().map_err(|e| {
        error_to_response("Failed to get DB connection from pool", &e.to_string())
    })?;

    proposal_repository::find_proposal_by_id(&mut conn, id)
        .map(|proposal| HttpResponse::Ok().json(proposal.to_response()))
        .map_err(|e| {
            error_to_response("Internal Server Error", &format!("An error occurred: {}", e))
        })
}

#[post("/proposals")]
async fn create_proposal(
    pool: web::Data<DbPool>,
    proposal_json: web::Json<ProposalRequest>,
) -> Result<HttpResponse, impl ResponseError> {
    let mut conn = pool.get().map_err(|e| {
        error_to_response("Failed to get DB connection from pool", &e.to_string())
    })?;

    let proposal = proposal_json.into_inner().to_entity();
    proposal_repository::create_proposal(&mut conn, &proposal)
        .map(|proposal| HttpResponse::Created().json(proposal.to_response()))
        .map_err(|e| {
            error_to_response("Internal Server Error", &format!("An error occurred: {}", e))
        })
}

#[derive(Deserialize)]
pub struct TakerIdQuery {
    taker_id: Uuid, // Ensure this field name matches the query parameter name you expect
}

#[derive(Deserialize)]
pub struct MakerOrTakerIdQuery {
    maker_or_taker_id: Uuid,
}

#[post("/proposals/{id}/considerations")]
async fn create_consideration(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    consideration_json: web::Json<ConsiderationRequest>,
    query: web::Query<TakerIdQuery>,
) -> Result<HttpResponse, impl ResponseError> {
    let taker_id = query.taker_id;
    let proposal_id = path.into_inner();
    let mut conn = pool.get().map_err(|e| {
        error_to_response("Failed to get DB connection from pool", &e.to_string())
    })?;

    let consideration = consideration_json.into_inner().to_entity(proposal_id, taker_id);
    consideration_repository::create_consideration(&mut conn, &consideration)
        .map(|consideration| HttpResponse::Created().json(consideration.to_response()))
        .map_err(|e| {
            error_to_response("Internal Server Error", &format!("An error occurred: {}", e))
        })
}

#[get("/proposals/{id}/considerations")]
async fn get_considerations_by_proposal_id(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    query: web::Query<MakerOrTakerIdQuery>,
) -> Result<HttpResponse, impl ResponseError> {
    let maker_or_taker_id = query.maker_or_taker_id;
    let proposal_id: Uuid = path.into_inner();
    let mut conn = pool.get().map_err(|e| {
        error_to_response("Failed to get DB connection from pool", &e.to_string())
    })?;

    consideration_repository::find_considerations_by_proposal_id(&mut conn, proposal_id)
        .map(|considerations| {
            let responses: Vec<ConsiderationResponse> = considerations.iter()
                .map(|consideration| consideration.to_response())
                .collect();
            HttpResponse::Ok().json(responses)
        })
        .map_err(|e| {
            error_to_response("Internal Server Error", &format!("An error occurred: {}", e))
        })
}

fn error_to_response(error: &str, message: &str) -> ErrorResponse {
    ErrorResponse::new(error, message)
}

pub async fn fallback_route(req: HttpRequest) -> impl Responder {
    let path = req.path();
    let response_message = format!("The requested route '{}' was not found.", path);
    HttpResponse::NotFound().body(response_message)
}
