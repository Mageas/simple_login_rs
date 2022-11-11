use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct OptionsData {
    pub can_create: bool,
    pub prefix_suggestion: String,
    pub suffixes: Vec<OptionsSuffixData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OptionsSuffixData {
    pub is_custom: bool,
    pub is_premium: bool,
    pub signed_suffix: String,
    pub suffix: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct VecAliasData {
    pub aliases: Vec<AliasData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AliasData {
    pub creation_date: String,
    pub creation_timestamp: usize,
    pub email: String,
    pub name: Option<String>,
    pub enabled: bool,
    pub id: usize,
    pub mailboxes: Vec<AliasMailboxData>,
    pub lastest_activity: Option<AliasLatestActivityData>,
    pub nb_block: usize,
    pub nb_forward: usize,
    pub nb_reply: usize,
    pub support_pgp: bool,
    pub disable_pgp: bool,
    pub note: Option<String>,
    pub pinned: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AliasMailboxData {
    pub email: String,
    pub id: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AliasLatestActivityData {
    pub action: String,
    pub contact: AliasLatestActivityContactData,
    pub timestamp: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AliasLatestActivityContactData {
    pub email: String,
    pub name: Option<String>,
    pub reverse_alias: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AliasToggleData {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct VecAliasActivityData {
    pub activities: Vec<AliasActivityData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AliasActivityData {
    pub action: String,
    pub from: String,
    pub timestamp: usize,
    pub to: String,
    pub reverse_alias: String,
    pub reverse_alias_address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VecAliasContactData {
    pub contacts: Vec<AliasContactData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AliasContactData {
    pub id: usize,
    pub contact: String,
    pub creation_date: String,
    pub creation_timestamp: usize,
    pub last_email_sent_date: Option<String>,
    pub last_email_sent_timestamp: Option<usize>,
    pub reverse_alias: String,
    pub reverse_alias_address: String,
    pub block_forward: bool,
    pub existed: bool,
}
