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
    match status.as_u16() {
        200 | 201 => Ok(()),
        400 | 403 => {
            let error = serde_json::from_str::<ErrorData>(body.as_ref())
                .map_err(|e| SimpleLoginError::DeserializeApiErrorResponse(e))?;

            Err(SimpleLoginError::ApiErrorResponse { error: error.error })
        }
        401 => Err(SimpleLoginError::BadCredentials),
        410 => Err(SimpleLoginError::TooManyWrongTries),
        440 => Err(SimpleLoginError::NeedSudo),
        _ => Err(SimpleLoginError::RequestStatusCode {
            path: path.to_string(),
            status,
        }),
    }
}
