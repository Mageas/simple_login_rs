use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SettingData {
    pub alias_generator: String,
    pub notification: bool,
    pub random_alias_default_domain: String,
    pub random_alias_suffix: String,
    pub sender_format: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SettingDomainData {
    pub domain: String,
    pub is_custom: bool,
}
