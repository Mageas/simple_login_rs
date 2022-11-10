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

    // todo: update &str to Enums (even in the return type)
    /// Update user's settings
    pub async fn update(
        self,
        alias_generator: Option<&str>,
        notification: Option<bool>,
        random_alias_default_domain: Option<&str>,
        random_alias_suffix: Option<&str>,
        sender_format: Option<&str>,
    ) -> SimpleLoginResult<SettingData> {
        let endpoint = "api/setting";

        #[derive(serde::Serialize)]
        struct Body<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            alias_generator: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            notification: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            random_alias_default_domain: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            random_alias_suffix: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sender_format: Option<&'a str>,
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
