use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct NotificationsData {
    pub more: bool,
    pub notifications: Vec<NotificationData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NotificationData {
    pub created_at: String,
    pub id: usize,
    pub message: String,
    pub read: bool,
}
