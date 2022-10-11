use serde::{Serialize, Deserialize};


/// The general purpose error response
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: bool,
    pub message: String
}