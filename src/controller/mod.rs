pub mod todo;

use poem_openapi::Tags;

#[derive(Tags)]
enum ApiTag {
    #[oai(rename = "Todo Controller")]
    TodoController,
}
