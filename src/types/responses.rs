// __LICENSE__

use poem_openapi::Object;
use poem_openapi::payload::Json;
use poem_openapi::types::{ParseFromJSON, ToJSON, Type};
use serde::{Deserialize, Serialize};

/// A generic response structure that can be used to wrap any type of data along with a message.
#[derive(Debug, Serialize, Deserialize, Object)]
pub struct BaseResponse<T: Type + ParseFromJSON + ToJSON> {
    pub data: T,
    pub success: bool,
}

impl<T: Type + ParseFromJSON + ToJSON> From<T> for BaseResponse<T> {
    fn from(data: T) -> Self {
        Self {
            data,
            success: true,
        }
    }
}

/// Generic response type structure
pub type BaseResponseType<T> = Json<BaseResponse<T>>;

/// Generic resposne structure for error responses.
#[derive(Debug, Serialize, Deserialize, Object)]
pub struct ErrorResponse {
    pub message: String,
    #[oai(default)]
    pub success: bool,
}

impl ErrorResponse {
    pub fn internal_default() -> Self {
        Self {
            message: "Internal Server Error".to_string(),
            success: false,
        }
    }
}

impl From<String> for ErrorResponse {
    fn from(message: String) -> Self {
        Self {
            message: message,
            success: false,
        }
    }
}

#[macro_export]
macro_rules! api_response {
    // Body, with description
    ($(#[$meta:meta])* $vis:vis $name:ident, $status:tt, $data:ty, $desc:literal) => {
        #[derive(::poem_openapi::ApiResponse)]
        $(#[$meta])*
        $vis enum $name {
            #[doc = $desc]
            #[oai(status = $status)]
            Success($crate::types::responses::BaseResponseType<$data>),
        }
    };

    // Body, no description
    ($(#[$meta:meta])* $vis:vis $name:ident, $status:tt, $data:ty) => {
        #[derive(::poem_openapi::ApiResponse)]
        $(#[$meta])*
        $vis enum $name {
            #[oai(status = $status)]
            Success($crate::types::responses::BaseResponseType<$data>),
        }
    };

    // No body, with description
    ($(#[$meta:meta])* $vis:vis $name:ident, $status:tt, $desc:literal) => {
        #[derive(::poem_openapi::ApiResponse)]
        $(#[$meta])*
        $vis enum $name {
            #[doc = $desc]
            #[oai(status = $status)]
            Success,
        }
    };

    // No body, no description
    ($(#[$meta:meta])* $vis:vis $name:ident, $status:tt) => {
        #[derive(::poem_openapi::ApiResponse)]
        $(#[$meta])*
        $vis enum $name {
            #[oai(status = $status)]
            Success,
        }
    };
}
