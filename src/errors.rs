use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimpleLoginError {
    #[error("Unable to request {1}")]
    GenericRequest(#[source] reqwest::Error, String),

    #[error("Bad credentials")]
    BadCredentials,

    #[error("{}", .error)]
    ApiErrorResponse { error: String },

    #[error("Token not set")]
    TokenNotSet,

    #[error("Unable to deserialize the error from the bad request")]
    DeserializeApiErrorResponse(#[source] serde_json::Error),

    #[error("Unable to deserialize the data")]
    DeserializeApiResponse(#[source] serde_json::Error),

    #[error("Unable mo request {} with status {}", .path, .status)]
    RequestStatusCode {
        // source: reqwest::Error,
        path: String,
        status: reqwest::StatusCode,
    },
}

pub type SimpleLoginResult<T = ()> = Result<T, SimpleLoginError>;
