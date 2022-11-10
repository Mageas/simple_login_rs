use std::collections::HashMap;

use crate::{
    notification::NotificationsData, BaseHttpClient, OkData, SimpleLoginError, SimpleLoginResult,
};

use super::SimpleLogin;

pub struct EndpointsNotification<'a, S: SimpleLogin>(pub(crate) &'a S);

impl<S: SimpleLogin> EndpointsNotification<'_, S> {
    /// Get notifications
    pub async fn list(self, page: usize) -> SimpleLoginResult<NotificationsData> {
        let endpoint = &format!("api/notifications");

        let page = page.to_string();

        let query = HashMap::from([("page", page.as_str())]);

        let response = self
            .0
            .get_http()
            .get(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(Some(&query), None),
            )
            .await?;

        serde_json::from_str::<NotificationsData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }

    // todo: unable to test the return type
    /// Mark as read a notification
    pub async fn read(self, notification_id: usize) -> SimpleLoginResult<OkData> {
        let endpoint = &format!("api/notifications/{notification_id}");

        let response = self
            .0
            .get_http()
            .post(
                self.0.get_token(),
                &self.0.get_url(&endpoint),
                &(None, None),
            )
            .await?;

        serde_json::from_str::<OkData>(&response)
            .map_err(|e| SimpleLoginError::DeserializeApiResponse(e))
    }
}
