use poem_openapi::{ApiResponse, Object};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct PostTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(ApiResponse, Debug)]
pub enum PostTodoResponseType {
    /// Yes! It works!
    #[oai(status = 204)]
    Success,
}
