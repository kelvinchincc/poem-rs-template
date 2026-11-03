use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct GetTodoResponse {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

pub type GetTodoResponses = Vec<GetTodoResponse>;
