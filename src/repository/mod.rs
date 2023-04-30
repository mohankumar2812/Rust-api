pub mod mongodb_repo;
use crate::models::{user_model::User, tokens::Token};

pub enum RegisterRequest {
    NewUser,
    AlreadyCreated,
}

pub enum LoginRequestError {
    Ok(User),
    UserNotFound,
}

pub enum JwtToken {
    Ok(Token),
    InvalidToken
}