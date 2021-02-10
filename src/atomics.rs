/// Atomic types handle validation and serialization of field types.
use fmt::Display;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub type BoxResult<T> = Result<T, Box<dyn Error>>;
pub type CouldBe<T> = BoxResult<Option<T>>;
pub type Worked = BoxResult<()>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Email(String);

impl Email {
    pub fn from_str(email: &str) -> Self {
        // Todo: add validation and regularization,
        // as in let email = email.trim().to_lowercase();
        Self(email.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Password(String);

impl Password {
    pub fn from_str(password: &str) -> Self {
        Self(password.to_string())
    }
    pub fn to_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Id(pub u64);

impl Id {
    pub fn from_u64(id: u64) -> Self {
        Self(id)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn from_str(password_hash: &str) -> Self {
        Self(password_hash.to_string())
    }
    pub fn to_str(&self) -> &str {
        &self.0
    }
}

use std::fmt;

#[derive(Debug)]
pub struct BasicError {
    details: String,
}

impl BasicError {
    pub fn new(msg: &str) -> BasicError {
        BasicError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for BasicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for BasicError {
    fn description(&self) -> &str {
        &self.details
    }
}

use actix_web::{dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse};

#[derive(Debug)]
pub struct StatusError {
    pub details: String,
    pub status_code: StatusCode,
}

impl StatusError {
    pub fn new(details: &str, status_code: StatusCode) -> Self {
        StatusError{
            details: details.to_owned(), status_code
        }
    }
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for StatusError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl error::ResponseError for StatusError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}