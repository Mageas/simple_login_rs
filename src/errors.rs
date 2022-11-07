use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimpleLoginError {
    #[error("Unable to request '{1}'")]
    Request(#[source] reqwest::Error, String),

    #[error("Unable to request '{}' with status {}", .path, .status)]
    RequestStatusCode {
        path: String,
        status: reqwest::StatusCode,
    },

    #[error("{}", .error)]
    ApiErrorResponse { error: String },

    #[error("Bad credentials")]
    BadCredentials,

    #[error("Token not set")]
    TokenNotSet,

    #[error("Sudo is needed to access this endpoint")]
    NeedSudo,

    #[error("Too many wrong tries, please ask for a reactivation 'api/auth/reactivate'")]
    TooManyWrongTries,

    #[error("Unable to deserialize the data from an error return by the api")]
    DeserializeApiErrorResponse(#[source] serde_json::Error),

    #[error("Unable to deserialize the data")]
    DeserializeApiResponse(#[source] serde_json::Error),
}

pub type SimpleLoginResult<T = ()> = Result<T, SimpleLoginError>;
