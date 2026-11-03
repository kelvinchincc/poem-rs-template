// demo of using the template
use color_eyre::Result;
use poem_openapi::{OpenApi, payload::Json, types::ToJSON};
use tracing::info;

use crate::{
    api_errors, api_response,
    controller::ApiTag,
    dto::{get_todo_api::GetTodoResponses, post_todo_api::PostTodoRequest},
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

        // Ok(PostTodoResponseType::Success)
        Err(ServiceError::InternalServerError(Some("Simulated".into())).into())
    }
}

api_errors!(pub GetTodoError { NotFound = 404 => "Todo item not found!" });
api_errors!(pub PostTodoError {});

api_response!(pub GetTodoResponseType, 200, GetTodoResponses, "200 Ok");
api_response!(pub PostTodoResponseType, 204, "Yes! It works!");
