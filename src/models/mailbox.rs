use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MailboxData {
    pub id: usize,
    pub email: String,
    pub verified: bool,
    pub default: bool,
    pub nb_alias: usize,
    pub creation_timestamp: usize,
}
