#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
}

#[macro_export]
macro_rules! api_errors {
    ($vis:vis $name:ident { $($variant:ident = $status:literal => $desc:literal),* $(,)? }) => {
        #[derive(Debug, poem_openapi::ApiResponse)]
        $vis enum $name {
            $(
                #[doc = $desc]
                #[oai(status = $status)]
                $variant(poem_openapi::payload::Json<$crate::types::responses::ErrorResponse>),
            )*
            /// Internal server error
            #[oai(status = 500)]
            InternalServerError,
        }

        impl From<ServiceError> for $name {
            fn from(e: ServiceError) -> Self {
                match e {
                    $( ServiceError::$variant(m) => Self::$variant(Json(m.into())), )*
                    other => {
                        log::error!("Internal server error: {other}");
                        Self::InternalServerError
                    }
                }
            }
        }
    };
}
