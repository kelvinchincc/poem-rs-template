// __LICENSE__

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Internal Server Error: {}", _0.as_deref().unwrap_or("Internal Server Error"))]
    InternalServerError(Option<String>),
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
            InternalServerError(poem_openapi::payload::Json<$crate::types::responses::ErrorResponse>),
        }

        impl From<$crate::error::service::ServiceError> for $name {
            fn from(e: $crate::error::service::ServiceError) -> Self {
                use $crate::error::service::ServiceError;
                use $crate::types::responses::ErrorResponse;
                use poem_openapi::payload::Json;

                match e {
                    $( ServiceError::$variant(m) => Self::$variant(Json(m.into())), )*

                    // Explicit 500 from the service layer: use its message, or the default
                    ServiceError::InternalServerError(m) => {
                        log::error!("Internal server error: {m:?}");
                        let body = match m {
                            Some(msg) => ErrorResponse::from(msg),
                            None => ErrorResponse::internal_default(),
                        };
                        Self::InternalServerError(Json(body))
                    }

                    // Any ServiceError variant this endpoint didn't declare
                    #[allow(unreachable_patterns)]
                    other => {
                        log::error!("Unmapped service error: {other}");
                        Self::InternalServerError(Json(ErrorResponse::internal_default()))
                    }
                }
            }
        }
    };
}
