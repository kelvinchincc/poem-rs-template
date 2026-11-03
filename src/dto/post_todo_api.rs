use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct PostTodoRequest {
    pub title: String,
    pub description: Option<String>,
}
