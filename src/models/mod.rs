use serde::Deserialize;

pub mod account;
pub mod alias;
pub mod contact;
pub mod custom_domain;
pub mod mailbox;
pub mod notification;
pub mod setting;

#[derive(Debug, Deserialize)]
pub struct ErrorData {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct MsgData {
    pub msg: String,
}

#[derive(Debug, Deserialize)]
pub struct OkData {
    pub ok: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdatedData {
    pub updated: bool,
}

#[derive(Debug, Deserialize)]
pub struct DeletedData {
    pub deleted: bool,
}
