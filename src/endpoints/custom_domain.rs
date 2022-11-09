use crate::{
    custom_domain::{
        CustomDomainData, DeCustomDomainData, DeletedAliasData, VecCustomDomainData,
        VecDeletedAliasData,
    },
    BaseHttpClient, SimpleLoginError, SimpleLoginResult,
};

use super::SimpleLogin;

pub struct EndpointsCustomDomain<'a, S: SimpleLogin>(pub(crate) &'a S);

impl<S: SimpleLogin> EndpointsCustomDomain<'_, S> {
    /// Get custom domains
    pub async fn list(self) -> SimpleLoginResult<Vec<CustomDomainData>> {
        let endpoint = "api/custom_domains";

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        Ok(serde_json::from_str::<VecCustomDomainData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))?
            .custom_domains)
    }

    /// Update custom domain's information
    pub async fn update(
        self,
        custom_domain_id: usize,
        catch_all: Option<bool>,
        random_prefix_generation: Option<bool>,
        name: Option<&str>,
        mailbox_ids: Option<&[usize]>,
    ) -> SimpleLoginResult<CustomDomainData> {
        let endpoint = &format!("api/custom_domains/{custom_domain_id}");

        #[derive(serde::Serialize)]
        struct Body<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            catch_all: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            random_prefix_generation: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            mailbox_ids: Option<&'a [usize]>,
        }

        let body = serde_json::to_value(Body {
            catch_all,
            random_prefix_generation,
            name,
            mailbox_ids,
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

        Ok(serde_json::from_str::<DeCustomDomainData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))?
            .custom_domain)
    }

    /// Get deleted aliases of a custom domain
    pub async fn trash(self, custom_domain_id: usize) -> SimpleLoginResult<Vec<DeletedAliasData>> {
        let endpoint = &format!("api/custom_domains/{custom_domain_id}/trash");

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        Ok(serde_json::from_str::<VecDeletedAliasData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))?
            .aliases)
    }
}
