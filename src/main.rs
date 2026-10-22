use color_eyre::Result;
use poem_openapi::OpenApiService;

mod controller;
mod dto;
mod error;
mod types;

const API_VERSION: &str = "1.0";
const API_TITLE: &str = "Todo API";
const API_IP: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> Result<()> {
    init_env()?;

    let api_service = OpenApiService::new(controller::todo::TodoController, API_TITLE, API_VERSION)
        .server("/api");
    let ui = api_service.swagger_ui();
    let app = poem::Route::new()
        .nest("/docs", ui)
        .nest("/api", api_service);
    poem::Server::new(poem::listener::TcpListener::bind(API_IP))
        .run(app)
        .await?;

    Ok(())
}

fn init_env() -> Result<()> {
    color_eyre::install()?;

    if std::env::var_os("RUST_LOG").is_none() {
        unsafe { std::env::set_var("RUST_LOG", "debug") };
    }

    tracing_subscriber::fmt::init();

    Ok(())
}
