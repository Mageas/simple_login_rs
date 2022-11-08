use serde_json::json;

use crate::{
    mailbox::MailboxData, BaseHttpClient, DeletedData, SimpleLoginError, SimpleLoginResult,
    UpdatedData,
};

use super::SimpleLogin;

pub struct EndpointsMailbox<'a, S: SimpleLogin>(pub(crate) &'a S);

impl<S: SimpleLogin> EndpointsMailbox<'_, S> {
    /// Create a new mailbox
    pub async fn create(self, email: &str) -> SimpleLoginResult<MailboxData> {
        let endpoint = "api/mailboxes";

        let body = json!({
            "email": email,
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

        serde_json::from_str::<MailboxData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    /// Delete a mailbox
    pub async fn delete(self, mailbox_id: &str) -> SimpleLoginResult<DeletedData> {
        let endpoint = &format!("api/mailboxes/{mailbox_id}");

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

    /// Update a mailbox
    pub async fn update(
        self,
        mailbox_id: &str,
        default: Option<bool>,
        email: Option<&str>,
        cancel_email_change: Option<bool>,
    ) -> SimpleLoginResult<UpdatedData> {
        let endpoint = &format!("api/mailboxes/{mailbox_id}");

        #[derive(serde::Serialize)]
        struct Body<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            default: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            email: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            cancel_email_change: Option<bool>,
        }

        let body = serde_json::to_value(Body {
            default,
            email,
            cancel_email_change,
        })
        .unwrap();

        let response = self
            .0
            .get_http()
            .put(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, Some(&body)),
            )
            .await?;

        serde_json::from_str::<UpdatedData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }
}
