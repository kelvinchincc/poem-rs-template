use poem_openapi::{ApiResponse, Object, payload::Json};
use serde::{Deserialize, Serialize};

use crate::types::responses::BaseResponse;

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct GetTodoResponse {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[derive(ApiResponse)]
pub enum GetTodoResponseType {
    /// Success response with a list of todo items
    #[oai(status = 200)]
    Success(Json<BaseResponse<Vec<GetTodoResponse>>>),
}
