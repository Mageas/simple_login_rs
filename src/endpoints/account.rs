use serde_json::json;

use crate::{
    account::{ApiKeyData, CookieTokenData, LoginData, MfaData, UserInfoData},
    BaseHttpClient, MsgData, OkData, SimpleLoginError, SimpleLoginResult,
};

use super::SimpleLogin;

pub struct EndpointsAccount<'a, S: SimpleLogin>(pub(crate) &'a S);

impl<S: SimpleLogin> EndpointsAccount<'_, S> {
    /// Authentication
    pub async fn login(
        self,
        email: &str,
        password: &str,
        device: &str,
    ) -> SimpleLoginResult<LoginData> {
        let endpoint = "api/auth/login";

        let body = json!({
            "email": email,
            "password": password,
            "device": device
        });

        let response = self
            .0
            .get_http()
            .post_public(&self.0.get_url(&endpoint), &(None, Some(&body)))
            .await?;

        serde_json::from_str::<LoginData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// 2FA authentication
    pub async fn mfa(
        self,
        mfa_token: &str,
        mfa_key: &str,
        device: &str,
    ) -> SimpleLoginResult<MfaData> {
        let endpoint = "api/auth/mfa";

        let body = json!({
            "mfa_token": mfa_token,
            "mfa_key": mfa_key,
            "device": device
        });

        let response = self
            .0
            .get_http()
            .post_public(&self.0.get_url(&endpoint), &(None, Some(&body)))
            .await?;

        serde_json::from_str::<MfaData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Register a new account
    pub async fn register(self, email: &str, password: &str) -> SimpleLoginResult<MsgData> {
        let endpoint = "api/auth/register";

        let body = json!({
            "email": email,
            "password": password,
        });

        let response = self
            .0
            .get_http()
            .post_public(&self.0.get_url(&endpoint), &(None, Some(&body)))
            .await?;

        serde_json::from_str::<MsgData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Activate new account
    pub async fn activate(self, email: &str, code: &str) -> SimpleLoginResult<MsgData> {
        let endpoint = "api/auth/activate";

        let body = json!({
            "email": email,
            "code": code,
        });

        let response = self
            .0
            .get_http()
            .post_public(&self.0.get_url(&endpoint), &(None, Some(&body)))
            .await?;

        serde_json::from_str::<MsgData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Request a new activation code
    pub async fn reactivate(self, email: &str) -> SimpleLoginResult<MsgData> {
        let endpoint = "api/auth/reactivate";

        let body = json!({
            "email": email,
        });

        let response = self
            .0
            .get_http()
            .post_public(&self.0.get_url(&endpoint), &(None, Some(&body)))
            .await?;

        serde_json::from_str::<MsgData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Request reset password link
    pub async fn forgot_password(self, email: &str) -> SimpleLoginResult<OkData> {
        let endpoint = "api/auth/forgot_password";

        let body = json!({
            "email": email,
        });

        let response = self
            .0
            .get_http()
            .post_public(&self.0.get_url(&endpoint), &(None, Some(&body)))
            .await?;

        serde_json::from_str::<OkData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Get user's information
    pub async fn get_user_info(self) -> SimpleLoginResult<UserInfoData> {
        let endpoint = "api/user_info";

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        serde_json::from_str::<UserInfoData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Enable sudo mode
    pub async fn sudo(self, password: &str) -> SimpleLoginResult<OkData> {
        let endpoint = "api/sudo";

        let body = json!({
            "password": password,
        });

        let response = self
            .0
            .get_http()
            .patch(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, Some(&body)),
            )
            .await?;

        serde_json::from_str::<OkData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Delete the current user
    pub async fn delete_user(self) -> SimpleLoginResult<OkData> {
        let endpoint = "api/user";

        let response = self
            .0
            .get_http()
            .delete(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        serde_json::from_str::<OkData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Get a one time use token to exchange it for a valid cookie
    pub async fn cookie_token(self) -> SimpleLoginResult<CookieTokenData> {
        let endpoint = "api/user/cookie_token";

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        serde_json::from_str::<CookieTokenData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Update user's information
    pub async fn update_user_info(
        self,
        profile_picture: (bool, Option<&str>),
        name: Option<&str>,
    ) -> SimpleLoginResult<UserInfoData> {
        let endpoint = "api/user_info";

        // Variant A
        #[derive(serde::Serialize)]
        struct BodyA<'a> {
            profile_picture: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<&'a str>,
        }

        // Variant B
        #[derive(serde::Serialize)]
        struct BodyB<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<&'a str>,
        }

        // Choose the variant
        let body = if profile_picture.0 {
            serde_json::to_value(BodyA {
                profile_picture: profile_picture.1,
                name,
            })
            .unwrap()
        } else {
            serde_json::to_value(BodyB { name }).unwrap()
        };

        let response = self
            .0
            .get_http()
            .patch(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, Some(&body)),
            )
            .await?;

        serde_json::from_str::<UserInfoData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Create a new API key
    pub async fn create_api_key(self, device: &str) -> SimpleLoginResult<ApiKeyData> {
        let endpoint = "api/api_key";

        let body = json!({
            "device": device,
        });

        let response = self
            .0
            .get_http()
            .post(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, Some(&body)),
            )
            .await?;

        serde_json::from_str::<ApiKeyData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Log out
    pub async fn logout(self) -> SimpleLoginResult<MsgData> {
        let endpoint = "api/logout";

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        serde_json::from_str::<MsgData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }
}
