use reqwest::StatusCode;

use crate::{ErrorData, SimpleLoginError, SimpleLoginResult};

pub(crate) async fn parse_error_from_response<S, D>(
    body: S,
    status: StatusCode,
    path: D,
) -> SimpleLoginResult
where
    S: AsRef<str>,
    D: AsRef<str> + std::fmt::Display,
{
    match status {
        StatusCode::OK => Ok(()),
        StatusCode::BAD_REQUEST => {
            let error = serde_json::from_str::<ErrorData>(body.as_ref())
                .map_err(|e| SimpleLoginError::DeserializeApiErrorResponse(e))?;

            Err(SimpleLoginError::ApiErrorResponse { error: error.error })
        }
        StatusCode::UNAUTHORIZED => Err(SimpleLoginError::BadCredentials),
        _ => Err(SimpleLoginError::RequestStatusCode {
            path: path.to_string(),
            status,
        }),
    }
}
