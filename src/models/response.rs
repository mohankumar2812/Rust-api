use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrResponse {
    pub message: String,
    pub status: bool,
}