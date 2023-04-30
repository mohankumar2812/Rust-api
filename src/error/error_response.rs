use crate::StatusCode;
use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub(crate) cause: &'static str,
}

pub const ERROR_MAIL_ALREADY_EXIST : StatusCode = StatusCode::BAD_REQUEST;
pub const MAIL_ALREADY_EXIST : ErrorResponse = ErrorResponse{
    cause: "Mail already registered."
};