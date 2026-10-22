use poem_openapi::{OpenApi, payload::Json, types::ToJSON};
use tracing::info;

use crate::{
    api_errors,
    controller::ApiTag,
    dto::{
        get_todo_api::GetTodoResponseType,
        post_todo_api::{PostTodoRequest, PostTodoResponseType},
    },
    error::service::ServiceError,
};

pub struct TodoController;

#[OpenApi(tag = "ApiTag::TodoController")]
impl TodoController {
    #[oai(path = "/todo", method = "get")]
    pub async fn get_todo(&self) -> Result<GetTodoResponseType, GetTodoError> {
        // Simulate a service error
        let service_error = ServiceError::NotFound("Todo item not found".to_string());
        Err(service_error.into())
    }

    #[oai(path = "/todo", method = "post")]
    pub async fn new_todo(
        &self,
        body: Json<PostTodoRequest>,
    ) -> Result<PostTodoResponseType, PostTodoError> {
        info!("body: {}", body.to_json().unwrap());
        Ok(PostTodoResponseType::Success)
    }
}

api_errors!(pub GetTodoError { NotFound = 404 => "Todo item not found!" });
api_errors!(pub PostTodoError {});
