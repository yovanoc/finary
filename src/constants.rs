use serde::{Deserialize, Serialize};

pub const API_ROOT: &str = "https://api.finary.com";

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub result: Option<T>,
    pub message: Option<String>,
    pub error: Option<String>,
}
