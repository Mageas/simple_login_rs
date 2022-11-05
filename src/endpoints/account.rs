use crate::account::{
    ActivateData, ApiKeyData, CookieTokenData, DeleteUserData, ForgotPasswordData, LoginData,
    LogoutData, MfaData, ReactivateData, RegisterData, SudoData, UserInfoData,
};
use crate::{SimpleLoginError, SimpleLoginResult};

use super::utils;
use super::SimpleLogin;

pub struct EndpointsAccount<'a, S: SimpleLogin>(pub(crate) &'a S);

impl<S: SimpleLogin> EndpointsAccount<'_, S> {
    /// api/auth/login
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

    /// api/auth/mfa
    pub async fn mfa(
        self,
        mfa_token: &str,
        mfa_key: &str,
        device: &str,
    ) -> SimpleLoginResult<MfaData> {
        let endpoint = "api/auth/mfa";

        let response = self
            .0
            .get_http()
            .post(self.0.get_url(endpoint))
            .json(&std::collections::HashMap::from([
                ("mfa_token", mfa_token),
                ("mfa_key", mfa_key),
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

        serde_json::from_str::<MfaData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/auth/register
    pub async fn register(self, email: &str, password: &str) -> SimpleLoginResult<RegisterData> {
        let endpoint = "api/auth/register";

        let response = self
            .0
            .get_http()
            .post(self.0.get_url(endpoint))
            .json(&std::collections::HashMap::from([
                ("email", email),
                ("password", password),
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

        serde_json::from_str::<RegisterData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/auth/activate
    pub async fn activate(self, email: &str, code: &str) -> SimpleLoginResult<ActivateData> {
        let endpoint = "api/auth/activate";

        let response = self
            .0
            .get_http()
            .post(self.0.get_url(endpoint))
            .json(&std::collections::HashMap::from([
                ("email", email),
                ("code", code),
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

        serde_json::from_str::<ActivateData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/auth/reactivate
    pub async fn reactivate(self, email: &str) -> SimpleLoginResult<ReactivateData> {
        let endpoint = "api/auth/reactivate";

        let response = self
            .0
            .get_http()
            .post(self.0.get_url(endpoint))
            .json(&std::collections::HashMap::from([("email", email)]))
            .send()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        utils::parse_error_from_response(&body, status, endpoint).await?;

        serde_json::from_str::<ReactivateData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/auth/forgot_password
    pub async fn forgot_password(self, email: &str) -> SimpleLoginResult<ForgotPasswordData> {
        let endpoint = "api/auth/forgot_password";

        let response = self
            .0
            .get_http()
            .post(self.0.get_url(endpoint))
            .json(&std::collections::HashMap::from([("email", email)]))
            .send()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        utils::parse_error_from_response(&body, status, endpoint).await?;

        serde_json::from_str::<ForgotPasswordData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/sudo
    pub async fn sudo(self, password: &str) -> SimpleLoginResult<SudoData> {
        let endpoint = "api/sudo";

        let token = self.0.get_token().ok_or(SimpleLoginError::TokenNotSet)?;

        let response = self
            .0
            .get_http()
            .patch(self.0.get_url(endpoint))
            .header("Authentication", token)
            .json(&std::collections::HashMap::from([("password", password)]))
            .send()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        utils::parse_error_from_response(&body, status, endpoint).await?;

        serde_json::from_str::<SudoData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/user
    pub async fn delete_user(self) -> SimpleLoginResult<DeleteUserData> {
        let endpoint = "api/user";

        let token = self.0.get_token().ok_or(SimpleLoginError::TokenNotSet)?;

        let response = self
            .0
            .get_http()
            .delete(self.0.get_url(endpoint))
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

        serde_json::from_str::<DeleteUserData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/user/cookie_token
    pub async fn cookie_token(self) -> SimpleLoginResult<CookieTokenData> {
        let endpoint = "api/user/cookie_token";

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

        serde_json::from_str::<CookieTokenData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/user_info
    pub async fn get_user_info(self) -> SimpleLoginResult<UserInfoData> {
        let endpoint = "api/user_info";

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

    /// api/user_infos
    pub async fn update_user_info(
        self,
        profile_picture: Option<&str>,
        name: Option<&str>,
    ) -> SimpleLoginResult<UserInfoData> {
        let endpoint = "api/user_info";

        let token = self.0.get_token().ok_or(SimpleLoginError::TokenNotSet)?;

        let response = self
            .0
            .get_http()
            .patch(self.0.get_url(endpoint))
            .header("Authentication", token)
            .json(&std::collections::HashMap::from([
                ("profile_picture", profile_picture),
                ("name", name),
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

        serde_json::from_str::<UserInfoData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/api_key
    pub async fn api_key(self, device: &str) -> SimpleLoginResult<ApiKeyData> {
        let endpoint = "api/api_key";

        let token = self.0.get_token().ok_or(SimpleLoginError::TokenNotSet)?;

        let response = self
            .0
            .get_http()
            .post(self.0.get_url(endpoint))
            .header("Authentication", token)
            .json(&std::collections::HashMap::from([("device", device)]))
            .send()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        utils::parse_error_from_response(&body, status, endpoint).await?;

        serde_json::from_str::<ApiKeyData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/logout
    pub async fn logout(self) -> SimpleLoginResult<LogoutData> {
        let endpoint = "api/logout";

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

        serde_json::from_str::<LogoutData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }
}
