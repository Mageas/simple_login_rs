use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OptionData {
    pub can_create: bool,
    pub prefix_suggestion: String,
    pub suffixes: Vec<OptionSuffixData>,
}

#[derive(Debug, Deserialize)]
pub struct OptionSuffixData {
    pub is_custom: bool,
    pub is_premium: bool,
    pub signed_suffix: String,
    pub suffix: String,
}

#[derive(Debug, Deserialize)]
pub struct AliasesData {
    pub aliases: Vec<AliasData>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct AliasMailboxData {
    pub email: String,
    pub id: usize,
}

#[derive(Debug, Deserialize)]
pub struct AliasLatestActivityData {
    pub action: String,
    pub contact: AliasLatestActivityContactData,
    pub timestamp: usize,
}

#[derive(Debug, Deserialize)]
pub struct AliasLatestActivityContactData {
    pub email: String,
    pub name: Option<String>,
    pub reverse_alias: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteAliasData {
    pub deleted: bool,
}

#[derive(Debug, Deserialize)]
pub struct AliasToggleData {
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct AliasActivityData {
    pub action: String,
    pub from: String,
    pub timestamp: usize,
    pub to: String,
    pub reverse_alias: String,
    pub reverse_alias_address: String,
}

#[derive(Debug, Deserialize)]
pub struct AliasActivitiesData {
    pub activities: Vec<AliasActivityData>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAliasData {
    pub ok: bool,
}

#[derive(Debug, Deserialize)]
pub struct AliasConcactData {
    pub id: usize,
    pub contact: String,
    pub creation_date: String,
    pub creation_timestamp: usize,
    pub last_email_sent_date: Option<String>,
    pub last_email_sent_timestamp: Option<usize>,
    pub reverse_alias: String,
    pub block_forward: bool,
}

#[derive(Debug, Deserialize)]
pub struct AliasConcactsData {
    pub contacts: Vec<AliasConcactData>,
}

#[derive(Debug, Deserialize)]
pub struct AliasUpdateConcactData {
    pub id: usize,
    pub contact: String,
    pub creation_date: String,
    pub creation_timestamp: usize,
    pub last_email_sent_date: Option<String>,
    pub last_email_sent_timestamp: Option<usize>,
    pub reverse_alias: String,
    pub reverse_alias_address: String,
    pub existed: bool,
}
