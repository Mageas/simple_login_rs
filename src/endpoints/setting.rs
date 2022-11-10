use crate::{
    setting::{SettingData, SettingDomainData},
    BaseHttpClient, SimpleLoginError, SimpleLoginResult,
};

use super::SimpleLogin;

pub struct EndpointsSetting<'a, S: SimpleLogin>(pub(crate) &'a S);

impl<S: SimpleLogin> EndpointsSetting<'_, S> {
    /// Get user's settings
    pub async fn get(self) -> SimpleLoginResult<SettingData> {
        let endpoint = "api/setting";

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        serde_json::from_str::<SettingData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Update user's settings
    pub async fn update(
        self,
        alias_generator: Option<AliasGenerator>,
        notification: Option<bool>,
        random_alias_default_domain: Option<&str>,
        random_alias_suffix: Option<AliasRandomAliasSuffix>,
        sender_format: Option<AliasSenderFormat>,
    ) -> SimpleLoginResult<SettingData> {
        let endpoint = "api/setting";

        #[derive(serde::Serialize)]
        struct Body<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            alias_generator: Option<AliasGenerator>,
            #[serde(skip_serializing_if = "Option::is_none")]
            notification: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            random_alias_default_domain: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            random_alias_suffix: Option<AliasRandomAliasSuffix>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sender_format: Option<AliasSenderFormat>,
        }

        let body = serde_json::to_value(Body {
            alias_generator,
            notification,
            random_alias_default_domain,
            random_alias_suffix,
            sender_format,
        })
        .unwrap();

        let response = self
            .0
            .get_http()
            .patch(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, Some(&body)),
            )
            .await?;

        serde_json::from_str::<SettingData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Get domains that user can use to create random alias
    pub async fn domains(self) -> SimpleLoginResult<Vec<SettingDomainData>> {
        let endpoint = "api/v2/setting/domains";

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        serde_json::from_str::<Vec<SettingDomainData>>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }
}

#[derive(serde::Serialize)]
pub enum AliasGenerator {
    #[serde(rename = "uuid")]
    Uuid,
    #[serde(rename = "word")]
    Word,
}

#[derive(serde::Serialize)]
pub enum AliasRandomAliasSuffix {
    #[serde(rename = "word")]
    Word,
    #[serde(rename = "random_string")]
    RandomString,
}

#[derive(serde::Serialize)]
pub enum AliasSenderFormat {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "A")]
    A,
    #[serde(rename = "NAME_ONLY")]
    NameOnly,
    #[serde(rename = "AT_ONLY")]
    AtOnly,
    #[serde(rename = "NO_NAME")]
    NoName,
}
