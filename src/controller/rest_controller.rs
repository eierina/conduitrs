use std::ops::Deref;
use actix_web::{web, HttpResponse, Error, http::StatusCode, HttpRequest, Responder, post, get, ResponseError};
use diesel::{PgConnection, r2d2};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::{Error as DieselError, DatabaseErrorKind};
use serde::Deserialize;
use uuid::Uuid;
use crate::dto::agreement_request::AgreementRequest;
use crate::dto::agreement_response::AgreementResponse;
use crate::dto::consideration_request::ConsiderationRequest;
use crate::dto::consideration_response::ConsiderationResponse;


use crate::dto::error_response::ErrorResponse;
use crate::dto::proposal_request::ProposalRequest;
use crate::repository::{proposal_repository, consideration_repository, agreement_repository};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn get_db_conn(pool: &web::Data<DbPool>) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
    pool.get()
        .map_err(|_| {
            actix_web::error::ErrorInternalServerError(ErrorResponse::new(
                "Connection Error",
                "Failed to get DB connection from pool",
            ))
        })
}

#[derive(Deserialize)]
pub struct TakerIdQuery {
    taker_id: Uuid,
}

#[derive(Deserialize)]
pub struct MakerIdQuery {
    maker_id: Uuid,
}

#[derive(Deserialize)]
pub struct MakerOrTakerIdQuery {
    maker_or_taker_id: Uuid,
}

/// Converts Diesel errors to appropriate HTTP responses.
fn handle_diesel_error(e: DieselError) -> Error {
    match e {
        DieselError::NotFound => {
            ErrorResponse::new("Not Found", "Resource not found").with_status(StatusCode::NOT_FOUND).into()
        },
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            ErrorResponse::new("Conflict", "Resource already exists").with_status(StatusCode::CONFLICT).into()
        },
        _ => {
            ErrorResponse::new("Internal Server Error", "An unexpected error occurred").with_status(StatusCode::INTERNAL_SERVER_ERROR).into()
        }
    }
}

/// Attempts to retrieve a proposal by ID, or returns an error response.
#[get("/proposals/{id}")]
pub async fn get_proposal(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let mut conn = get_db_conn(&pool)?;

    proposal_repository::find_proposal_by_id(&mut conn, id)
        .map(|proposal| HttpResponse::Ok().json(proposal.to_response()))
        .map_err(handle_diesel_error)
}

/// Creates a new proposal or returns an error response.
#[post("/proposals")]
pub async fn create_proposal(
    pool: web::Data<DbPool>,
    proposal_json: web::Json<ProposalRequest>,
) -> Result<HttpResponse, Error> {
    let proposal = proposal_json.into_inner().to_entity();
    let mut conn = get_db_conn(&pool)?;

    proposal_repository::insert_proposal(&mut conn, &proposal)
        .map(|proposal| HttpResponse::Created().json(proposal.to_response()))
        .map_err(handle_diesel_error)
}

/// Creates a new consideration for a specific proposal, or returns an error response.
#[post("/proposals/{id}/considerations")]
pub async fn create_consideration(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    consideration_json: web::Json<ConsiderationRequest>,
    query: web::Query<TakerIdQuery>
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let consideration = consideration_json.into_inner().to_entity(id, query.taker_id);
    let mut conn = get_db_conn(&pool)?;

    consideration_repository::insert_consideration(&mut conn, &consideration)
        .map(|consideration| HttpResponse::Created().json(consideration.to_response()))
        .map_err(handle_diesel_error)
}

/// Creates a new agreement for a specific consideration, or returns an error response.
#[post("/considerations/{id}/agreement")]
pub async fn create_agreement_for_consideration(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    agreement_json: web::Json<AgreementRequest>,
    query: web::Query<MakerIdQuery>
) -> Result<HttpResponse, Error> {
    let consideration_id = path.into_inner();
    let mut conn = get_db_conn(&pool)?;
    let consideration = consideration_repository::find_consideration_by_id(&mut conn, consideration_id)
        .map_err(handle_diesel_error)?;

    let agreement = agreement_json.into_inner().to_entity(consideration_id, query.maker_id, consideration.taker_id);

    agreement_repository::insert_agreement(&mut conn, &agreement)
        .map(|agreement| HttpResponse::Created().json(agreement.to_response())) // Assuming Agreement has a to_response method
        .map_err(handle_diesel_error)
}


/// Retrieves all considerations for a given proposal ID, or returns an error response.
#[get("/proposals/{id}/considerations")]
pub async fn get_considerations_by_proposal_id(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let mut conn = get_db_conn(&pool)?;

    consideration_repository::find_considerations_by_proposal_id(&mut conn, id)
        .map(|considerations|{
            let responses: Vec<ConsiderationResponse> = considerations.iter()
                .map(|consideration| consideration.to_response())
                .collect();
            HttpResponse::Ok().json(responses)
        })
        .map_err(handle_diesel_error)
}

/// Retrieves all agreements for a given proposal ID, or returns an error response.
#[get("/proposals/{id}/agreements")]
pub async fn get_agreements_by_proposal_id(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let proposal_id = path.into_inner();
    let mut conn = get_db_conn(&pool)?;

    // Use the repository function to find agreements by proposal ID
    agreement_repository::find_agreements_by_proposal_id(&mut conn, proposal_id)
        .map(|agreements| {
            // Convert each Agreement entity to its response representation
            // Note: You might need a `to_response` method for Agreement similar to what you have for Consideration
            let responses: Vec<AgreementResponse> = agreements.iter()
                .map(|agreement| agreement.to_response()) // Assuming `to_response` exists for Agreement
                .collect();
            HttpResponse::Ok().json(responses)
        })
        .map_err(handle_diesel_error)
}

pub async fn fallback_route(req: HttpRequest) -> HttpResponse {
    let path = req.path();
    let response_message = format!("The requested route '{}' was not found.", path);
    ErrorResponse::new("Not Found", &response_message)
        .with_status(StatusCode::NOT_FOUND)
        .error_response()
}
