use crate::alias::AliasData;
use crate::alias::AliasesData;
use crate::alias::OptionData;
use crate::SimpleLoginError;
use crate::SimpleLoginResult;

use super::utils;
use super::SimpleLogin;

pub struct EndpointsAlias<'a, S: SimpleLogin>(pub(crate) &'a S);

impl<S: SimpleLogin> EndpointsAlias<'_, S> {
    /// api/v5/alias/options
    pub async fn option(self) -> SimpleLoginResult<OptionData> {
        let endpoint = "api/v5/alias/options";

        let token = self.0.get_token().ok_or(SimpleLoginError::TokenNotSet)?;

        let response = self
            .0
            .get_http()
            .get(self.0.get_url(endpoint))
            .header("Authentication", token)
            .json(&std::collections::HashMap::from([(
                "hostname",
                self.0.get_hostname(),
            )]))
            .send()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        utils::parse_error_from_response(&body, status, endpoint).await?;

        serde_json::from_str::<OptionData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/v3/alias/custom/new
    pub async fn create_custom(
        self,
        alias_prefix: &str,
        signed_prefix: &str,
        mailbox_ids: Vec<String>,
        note: Option<&str>,
        name: Option<&str>,
    ) -> SimpleLoginResult {
        let endpoint = "api/v3/alias/custom/new";

        let token = self.0.get_token().ok_or(SimpleLoginError::TokenNotSet)?;

        #[derive(Debug, serde::Serialize)]
        struct Body {
            hostname: String,
            alias_prefix: String,
            signed_prefix: String,
            mailbox_ids: Vec<String>,
            note: Option<String>,
            name: Option<String>,
        }

        let response = self
            .0
            .get_http()
            .post(self.0.get_url(endpoint))
            .header("Authentication", token)
            .json(&Body {
                hostname: self.0.get_hostname().into(),
                alias_prefix: alias_prefix.into(),
                signed_prefix: signed_prefix.into(),
                mailbox_ids: mailbox_ids,
                note: None,
                name: None,
            })
            // .json(&std::collections::HashMap::from([
            //     ("hostname", self.0.get_hostname()),
            //     ("alias_prefix", alias_prefix),
            //     ("signed_prefix", signed_prefix),
            // ]))
            // .json(&std::collections::HashMap::from([(
            //     "mailbox_ids",
            //     mailbox_ids,
            // )]))
            // .json(&std::collections::HashMap::from([
            //     ("note", note),
            //     ("name", name),
            // ]))
            .send()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        dbg!(&body);

        utils::parse_error_from_response(&body, status, endpoint).await?;

        todo!();
    }

    /// api/alias/random/new    
    pub async fn create_random(
        self,
        mode: Option<&str>,
        note: Option<&str>,
    ) -> SimpleLoginResult<AliasData> {
        let endpoint = "api/alias/random/new";

        let token = self.0.get_token().ok_or(SimpleLoginError::TokenNotSet)?;

        let response = self
            .0
            .get_http()
            .post(self.0.get_url(endpoint))
            .header("Authentication", token)
            .json(&std::collections::HashMap::from([(
                "hostname",
                self.0.get_hostname(),
            )]))
            .json(&std::collections::HashMap::from([
                ("mode", mode),
                ("note", note),
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

        serde_json::from_str::<AliasData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/v2/aliases
    pub async fn get_aliases(self, page_id: usize, filter: &str) -> SimpleLoginResult<AliasesData> {
        let endpoint = "api/v2/aliases";

        let token = self.0.get_token().ok_or(SimpleLoginError::TokenNotSet)?;

        let response = self
            .0
            .get_http()
            .get(self.0.get_url(endpoint))
            .header("Authentication", token)
            .query(&[("page_id", page_id)])
            .query(&[(filter, "")])
            .send()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        utils::parse_error_from_response(&body, status, endpoint).await?;

        serde_json::from_str::<AliasesData>(&body)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }
}
