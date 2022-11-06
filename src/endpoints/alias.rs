use std::collections::HashMap;

use crate::alias::AliasActivitiesData;
use crate::alias::AliasConcactsData;
use crate::alias::AliasData;
use crate::alias::AliasToggleData;
use crate::alias::AliasUpdateConcactData;
use crate::alias::AliasesData;
use crate::alias::DeleteAliasData;
use crate::alias::OptionData;
use crate::alias::UpdateAliasData;
use crate::SimpleLoginError;
use crate::SimpleLoginResult;

use super::SimpleLogin;

pub struct EndpointsAlias<'a, S: SimpleLogin>(pub(crate) &'a S);

impl<S: SimpleLogin> EndpointsAlias<'_, S> {
    /// api/v5/alias/options
    pub async fn option(self) -> SimpleLoginResult<OptionData> {
        let endpoint = "api/v5/alias/options";

        let query = HashMap::from([("hostname", self.0.get_hostname())]);

        let response = self.0.get::<&str, _>(endpoint, None, Some(query)).await?;

        serde_json::from_str::<OptionData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/v3/alias/custom/new
    pub async fn create_custom(
        self,
        alias_prefix: &str,
        signed_suffix: &str,
        mailbox_ids: &[&str],
        note: Option<&str>,
        name: Option<&str>,
    ) -> SimpleLoginResult<AliasData> {
        let endpoint = "api/v3/alias/custom/new";

        #[derive(serde::Serialize)]
        struct Body<'a> {
            hostname: &'a str,
            alias_prefix: &'a str,
            signed_suffix: &'a str,
            mailbox_ids: &'a [&'a str],
            #[serde(skip_serializing_if = "Option::is_none")]
            note: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<&'a str>,
        }

        let response = self
            .0
            .post::<_, &str>(
                endpoint,
                Some(Body {
                    hostname: self.0.get_hostname(),
                    alias_prefix: alias_prefix,
                    signed_suffix: signed_suffix,
                    mailbox_ids: mailbox_ids,
                    note,
                    name,
                }),
                None,
            )
            .await?;

        serde_json::from_str::<AliasData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/alias/random/new    
    pub async fn create_random(
        self,
        mode: Option<&str>,
        note: Option<&str>,
    ) -> SimpleLoginResult<AliasData> {
        let endpoint = "api/alias/random/new";

        #[derive(serde::Serialize)]
        struct Body<'a> {
            hostname: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            mode: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            note: Option<&'a str>,
        }

        let response = self
            .0
            .post::<_, &str>(
                endpoint,
                Some(Body {
                    hostname: self.0.get_hostname(),
                    mode,
                    note,
                }),
                None,
            )
            .await?;

        serde_json::from_str::<AliasData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/v2/aliases
    pub async fn list_aliases(
        self,
        page_id: usize,
        filter: &str,
    ) -> SimpleLoginResult<AliasesData> {
        let endpoint = "api/v2/aliases";

        let query = HashMap::from([("page_id", page_id.to_string()), (filter, "".to_string())]);

        let response = self.0.get::<&str, _>(endpoint, None, Some(query)).await?;

        serde_json::from_str::<AliasesData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/aliases/:alias_id
    pub async fn get_alias(self, alias_id: usize) -> SimpleLoginResult<AliasData> {
        let endpoint = &format!("api/aliases/{}", alias_id);

        let response = self.0.get::<&str, &str>(endpoint, None, None).await?;

        serde_json::from_str::<AliasData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/aliases/:alias_id
    pub async fn delete_alias(self, alias_id: usize) -> SimpleLoginResult<DeleteAliasData> {
        let endpoint = &format!("api/aliases/{}", alias_id);

        let response = self.0.delete::<&str, &str>(endpoint, None, None).await?;

        serde_json::from_str::<DeleteAliasData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/aliases/:alias_id/toggle
    pub async fn toggle_alias(self, alias_id: usize) -> SimpleLoginResult<AliasToggleData> {
        let endpoint = &format!("api/aliases/{}/toggle", alias_id);

        let response = self.0.post::<&str, &str>(endpoint, None, None).await?;

        serde_json::from_str::<AliasToggleData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/aliases/:alias_id/activities
    pub async fn alias_activities(
        self,
        page_id: usize,
        alias_id: usize,
    ) -> SimpleLoginResult<AliasActivitiesData> {
        let endpoint = &format!("api/aliases/{}/activities", alias_id);

        let query = HashMap::from([("page_id", page_id)]);

        let response = self.0.get::<&str, _>(&endpoint, None, Some(query)).await?;

        serde_json::from_str::<AliasActivitiesData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/aliases/:alias_id
    pub async fn update_alias(
        self,
        alias_id: usize,
        note: Option<&str>,
        name: Option<&str>,
        mailbox_ids: Option<&[&str]>,
        disable_pgp: Option<bool>,
        pinned: Option<bool>,
    ) -> SimpleLoginResult<UpdateAliasData> {
        let endpoint = &format!("api/aliases/{}", alias_id);

        #[derive(serde::Serialize)]
        struct Body<'a> {
            alias_id: usize,
            #[serde(skip_serializing_if = "Option::is_none")]
            note: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            mailbox_ids: Option<&'a [&'a str]>,
            #[serde(skip_serializing_if = "Option::is_none")]
            disable_pgp: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pinned: Option<bool>,
        }

        let response = self
            .0
            .patch::<_, &str>(
                &endpoint,
                Some(Body {
                    alias_id,
                    note,
                    name,
                    mailbox_ids,
                    disable_pgp,
                    pinned,
                }),
                None,
            )
            .await?;

        serde_json::from_str::<UpdateAliasData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/aliases/:alias_id/contacts
    pub async fn alias_contacts(
        self,
        page_id: usize,
        alias_id: usize,
    ) -> SimpleLoginResult<AliasConcactsData> {
        let endpoint = &format!("api/aliases/{}/contacts", alias_id);

        let query = HashMap::from([("page_id", page_id)]);

        let response = self.0.get::<&str, _>(&endpoint, None, Some(query)).await?;

        serde_json::from_str::<AliasConcactsData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// api/aliases/:alias_id/contacts
    pub async fn create_concact(
        self,
        alias_id: usize,
        contact: &str,
    ) -> SimpleLoginResult<AliasUpdateConcactData> {
        let endpoint = &format!("api/aliases/{}/contacts", alias_id);

        let body = HashMap::from([("contact", contact)]);

        let response = self.0.post::<_, &str>(&endpoint, Some(body), None).await?;

        serde_json::from_str::<AliasUpdateConcactData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }
}
