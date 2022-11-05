use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub api_key: String,
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
