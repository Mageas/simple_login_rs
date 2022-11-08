use std::collections::HashMap;

use serde_json::json;

use crate::alias::AliasActivityData;
use crate::alias::AliasContactData;
use crate::alias::AliasData;
use crate::alias::AliasToggleData;
use crate::alias::OptionsData;
use crate::DeletedData;
use crate::OkData;
use crate::SimpleLoginError;
use crate::SimpleLoginResult;

use crate::alias::VecAliasActivityData;
use crate::alias::VecAliasContactData;
use crate::alias::VecAliasData;
use crate::BaseHttpClient;

use super::SimpleLogin;

pub struct EndpointsAlias<'a, S: SimpleLogin>(pub(crate) &'a S);

impl<S: SimpleLogin> EndpointsAlias<'_, S> {
    /// Get alias options. Used by create alias process
    pub async fn options(self) -> SimpleLoginResult<OptionsData> {
        let endpoint = "api/v5/alias/options";

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        serde_json::from_str::<OptionsData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Create new alias
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

        let body = serde_json::to_value(Body {
            hostname: self.0.get_hostname(),
            alias_prefix,
            signed_suffix,
            mailbox_ids,
            note,
            name,
        })
        .unwrap();

        let response = self
            .0
            .get_http()
            .post(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, Some(&body)),
            )
            .await?;

        serde_json::from_str::<AliasData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    // todo: mode to Enum
    /// Random an alias
    pub async fn create_random(
        self,
        mode: Option<&str>,
        note: Option<&str>,
    ) -> SimpleLoginResult<AliasData> {
        let endpoint = "api/alias/random/new";

        #[derive(serde::Serialize)]
        struct Body<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            mode: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            note: Option<&'a str>,
        }

        let body = serde_json::to_value(Body { mode, note }).unwrap();

        let response = self
            .0
            .get_http()
            .post(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, Some(&body)),
            )
            .await?;

        serde_json::from_str::<AliasData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    // todo: filter to Enum
    // todo: update hashmap type
    /// Get user's aliases
    pub async fn list(self, page_id: usize, filter: &str) -> SimpleLoginResult<Vec<AliasData>> {
        let endpoint = "api/v2/aliases";

        let page_id = page_id.to_string();

        let query = HashMap::from([("page_id", page_id.as_str()), (filter, "")]);

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(Some(&query), None),
            )
            .await?;

        Ok(serde_json::from_str::<VecAliasData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))?
            .aliases)
    }

    /// Get alias information
    pub async fn get(self, alias_id: usize) -> SimpleLoginResult<AliasData> {
        let endpoint = &format!("api/aliases/{alias_id}");

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        serde_json::from_str::<AliasData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Delete an alias
    pub async fn delete(self, alias_id: usize) -> SimpleLoginResult<DeletedData> {
        let endpoint = &format!("api/aliases/{alias_id}");

        let response = self
            .0
            .get_http()
            .delete(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        serde_json::from_str::<DeletedData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Enable/disable an alias
    pub async fn toggle(self, alias_id: usize) -> SimpleLoginResult<AliasToggleData> {
        let endpoint = &format!("api/aliases/{alias_id}/toggle");

        let response = self
            .0
            .get_http()
            .post(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        serde_json::from_str::<AliasToggleData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    // todo: update hashmap type
    /// Get alias activities
    pub async fn activities(
        self,
        page_id: usize,
        alias_id: usize,
    ) -> SimpleLoginResult<Vec<AliasActivityData>> {
        let endpoint = &format!("api/aliases/{alias_id}/activities");

        let page_id = page_id.to_string();

        let query = HashMap::from([("page_id", page_id.as_str())]);

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(Some(&query), None),
            )
            .await?;

        Ok(serde_json::from_str::<VecAliasActivityData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))?
            .activities)
    }

    /// Update alias information
    pub async fn update(
        self,
        alias_id: usize,
        note: Option<&str>,
        name: Option<&str>,
        mailbox_ids: Option<&[&str]>,
        disable_pgp: Option<bool>,
        pinned: Option<bool>,
    ) -> SimpleLoginResult<OkData> {
        let endpoint = &format!("api/aliases/{alias_id}");

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

        let body = serde_json::to_value(Body {
            alias_id,
            note,
            name,
            mailbox_ids,
            disable_pgp,
            pinned,
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

        serde_json::from_str::<OkData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    // todo: update hashmap type
    /// Get alias contacts
    pub async fn contacts(
        self,
        page_id: usize,
        alias_id: usize,
    ) -> SimpleLoginResult<Vec<AliasContactData>> {
        let endpoint = &format!("api/aliases/{alias_id}/contacts");

        let page_id = page_id.to_string();

        let query = HashMap::from([("page_id", page_id.as_str())]);

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(Some(&query), None),
            )
            .await?;

        Ok(serde_json::from_str::<VecAliasContactData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))?
            .contacts)
    }

    /// Create a new contact for an alias
    pub async fn create_contact(
        self,
        alias_id: usize,
        contact: &str,
    ) -> SimpleLoginResult<AliasContactData> {
        let endpoint = &format!("api/aliases/{alias_id}/contacts");

        let body = json!({
            "contact": contact,
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

        serde_json::from_str::<AliasContactData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }
}
