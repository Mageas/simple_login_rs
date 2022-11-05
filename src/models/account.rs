use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub api_key: Option<String>,
    pub email: String,
    pub mfa_enabled: bool,
    pub mfa_key: Option<String>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoData {
    pub connected_proton_address: Option<String>,
    pub email: String,
    pub in_trial: bool,
    pub is_premium: bool,
    pub max_alias_free_plan: usize,
    pub profile_picture_url: Option<String>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct MfaData {
    pub api_key: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterData {
    pub msg: String,
}

#[derive(Debug, Deserialize)]
pub struct ActivateData {
    pub msg: String,
}

#[derive(Debug, Deserialize)]
pub struct ReactivateData {
    pub msg: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordData {
    pub ok: bool,
}

#[derive(Debug, Deserialize)]
pub struct SudoData {
    pub ok: bool,
}

#[derive(Debug, Deserialize)]
pub struct DeleteUserData {
    pub ok: bool,
}

#[derive(Debug, Deserialize)]
pub struct CookieTokenData {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiKeyData {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutData {
    pub msg: String,
}
