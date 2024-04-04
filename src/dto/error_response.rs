use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    //timestamp: String,
    //status: u16,
    pub error: String,
    pub message: String,
    //path: String,
}

impl ErrorResponse {
    pub fn new(error: &str, message: &str) -> Self {
        ErrorResponse {
            error: error.to_owned(),
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error, self.message)
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(ErrorResponse {
            error: self.error.clone(),
            message: self.message.clone(),
        })
    }
}