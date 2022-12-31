use crate::models::user_token::UserToken;
use actix_web::{error, http::StatusCode, HttpRequest, HttpResponse};
use std::fmt;

#[derive(Debug)]
pub enum VerifyError {
    MissingAuthorizationHeader,
    InvalidAuthorizationHeader,
    InvalidAuthorizationType,
}

impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerifyError::MissingAuthorizationHeader => {
                write!(f, "authorization header is missing")
            }
            VerifyError::InvalidAuthorizationHeader => {
                write!(f, "invalid authorization header")
            }
            VerifyError::InvalidAuthorizationType => {
                write!(f, "invalid authorization type")
            }
        }
    }
}

impl error::ResponseError for VerifyError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match *self {
            VerifyError::InvalidAuthorizationHeader => StatusCode::UNAUTHORIZED,
            VerifyError::MissingAuthorizationHeader => StatusCode::UNAUTHORIZED,
            VerifyError::InvalidAuthorizationType => StatusCode::UNAUTHORIZED,
        };

        HttpResponse::build(status_code).finish()
    }
}

pub fn signup() -> String {
    let token = UserToken::generate_token();
    return token;
}

pub fn verify(req: HttpRequest) -> Result<bool, VerifyError> {
    let authorization_header = match req.headers().get("authorization") {
        Some(header) => header.to_str().unwrap(),
        None => "",
    };

    if authorization_header.is_empty() {
        return Err(VerifyError::MissingAuthorizationHeader);
    }

    let authorization_header_split: Vec<&str> = authorization_header.split(" ").collect();

    if authorization_header_split.len() != 2 {
        return Err(VerifyError::InvalidAuthorizationHeader);
    }

    let auth_type = authorization_header_split[0];
    let auth_token = authorization_header_split[1];

    if auth_type != "Bearer" {
        return Err(VerifyError::InvalidAuthorizationType);
    }

    let is_valid = UserToken::verify_token(auth_token);

    return Ok(is_valid);
}
