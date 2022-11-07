use serde::Deserialize;

pub mod account;
pub mod alias;

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
