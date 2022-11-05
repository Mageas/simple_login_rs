use crate::account::{LoginData, UserInfoData};
use crate::{SimpleLoginError, SimpleLoginResult};

use super::utils;
use super::SimpleLogin;

pub struct EndpointsAccount<'a, S: SimpleLogin>(pub(crate) &'a S);

impl<S: SimpleLogin> EndpointsAccount<'_, S> {
    pub async fn login(
        self,
        email: &str,
        password: &str,
        device: &str,
    ) -> SimpleLoginResult<LoginData> {
        let endpoint = "api/auth/login";

        let response = self
            .0
            .get_http()
            .post(self.0.get_url(endpoint))
            .json(&std::collections::HashMap::from([
                ("email", email),
                ("password", password),
                ("device", device),
            ]))
            .send()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        utils::parse_error_from_response(&body, status, endpoint).await?;

        serde_json::from_str::<LoginData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    pub async fn user_info(self) -> SimpleLoginResult<UserInfoData> {
        let endpoint = "/api/user_info";

        let token = self.0.get_token().ok_or(SimpleLoginError::TokenNotSet)?;

        let response = self
            .0
            .get_http()
            .get(self.0.get_url(endpoint))
            .header("Authentication", token)
            .send()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        utils::parse_error_from_response(&body, status, endpoint).await?;

        serde_json::from_str::<UserInfoData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }
}

// pub(crate) async fn test<S, J>(
//     client: &reqwest::Client,
//     url: S,
//     body: J,
// ) -> SimpleLoginResult<reqwest::Response>
// where
//     S: AsRef<str> + reqwest::IntoUrl + Into<std::string::String>,
//     J: serde::ser::Serialize,
// {
//     client
//         .post(url.as_ref())
//         .json(&body)
//         .send()
//         .await
//         .map_err(|e| SimpleLoginError::GenericRequest(e, url.into()))
// }

// pub(crate) async fn parse_error_from_response(
//     response: reqwest::Response,
//     path: &str,
// ) -> SimpleLoginResult<reqwest::Response> {
//     let status_code = response.status().as_u16();
//     match status_code {
//         200 => Ok(response),
//         400 => {
//             let error = serde_json::from_str::<ErrorData>(
//                 &response
//                     .text()
//                     .await
//                     .map_err(|e| SimpleLoginError::GenericRequest(e, path.into()))?,
//             )
//             .map_err(|e| SimpleLoginError::DeserializeApiErrorResponse(e))?;

//             Err(SimpleLoginError::ApiErrorResponse { error: error.error })
//         }
//         401 => Err(SimpleLoginError::BadCredentials),
//         _ => Err(SimpleLoginError::RequestStatusCode {
//             path: path.into(),
//             status: response.status(),
//         }),
//     }
// }
