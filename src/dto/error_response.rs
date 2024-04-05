use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    //pub timestamp: String,
    pub error: String,
    pub message: String,
    status: u16,
    //path: String,
}

impl ErrorResponse {
    pub fn new(error: &str, message: &str) -> Self {
        ErrorResponse {
            error: error.to_owned(),
            message: message.to_owned(),
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), // Default status code
        }
    }

    // Change to modify the status inside the ErrorResponse and return `Self`
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status.as_u16();
        self
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error, self.message)
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        // Convert u16 back to StatusCode
        StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse {
        // Use the status_code() method to get the StatusCode for building the HttpResponse
        HttpResponse::build(self.status_code()).json(self)
    }
}
