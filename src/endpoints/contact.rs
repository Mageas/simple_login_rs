use crate::{
    contact::ToggleContactData, BaseHttpClient, DeletedData, SimpleLoginError, SimpleLoginResult,
};

use super::SimpleLogin;

pub struct EndpointsContact<'a, S: SimpleLogin>(pub(crate) &'a S);

impl<S: SimpleLogin> EndpointsContact<'_, S> {
    /// Delete a contact
    pub async fn delete(self, contact_id: usize) -> SimpleLoginResult<DeletedData> {
        let endpoint = &format!("api/contacts/{contact_id}");

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

    /// Toggle a contact
    pub async fn toggle(self, contact_id: usize) -> SimpleLoginResult<ToggleContactData> {
        let endpoint = &format!("api/contacts/{contact_id}/toggle");

        let response = self
            .0
            .get_http()
            .post(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        serde_json::from_str::<ToggleContactData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }
}
