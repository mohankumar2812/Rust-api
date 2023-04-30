use crate::error::error_response::{
    ErrorResponse, MAIL_ALREADY_EXIST, ERROR_MAIL_ALREADY_EXIST
};

use actix_web::http::StatusCode;
use actix_web::web::Json;

pub const ALREADY_REGISTERED_LOGIN: (StatusCode, Json<ErrorResponse>) = (
    ERROR_MAIL_ALREADY_EXIST,
    Json(MAIL_ALREADY_EXIST),
);

pub const JWT_EXPIRY: i64 = 3600;
pub const REFRESH_JWT_EXPIRY: i64 = 36400 * 24 * 30;

pub const JWT_SECRETKEY: &'static str = "jwt secretkey";
pub const REFRESH_JWT_SECRETKEY: &'static str = "jwt refresh secretkey";