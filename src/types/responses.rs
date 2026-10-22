use poem_openapi::Object;
use poem_openapi::types::{ParseFromJSON, ToJSON, Type};
use serde::{Deserialize, Serialize};

/// A generic response structure that can be used to wrap any type of data along with a message.
#[derive(Debug, Serialize, Deserialize, Object)]
pub struct BaseResponse<T: Type + ParseFromJSON + ToJSON> {
    pub data: T,
    pub message: String,
}

impl<T: Type + ParseFromJSON + ToJSON> From<T> for BaseResponse<T> {
    fn from(data: T) -> Self {
        Self {
            data,
            message: "Success".to_string(),
        }
    }
}

/// Generic resposne structure for error responses.
#[derive(Debug, Serialize, Deserialize, Object)]
pub struct ErrorResponse {
    pub message: String,
}

impl From<String> for ErrorResponse {
    fn from(message: String) -> Self {
        Self { message: message }
    }
}
