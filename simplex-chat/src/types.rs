use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocalProfile {
    pub profile_id: u64,
    pub display_name: String,
    pub full_name: String,
    pub image: Option<String>,
    pub contact_link: Option<String>,
    pub local_alias: String,
    #[serde(flatten)]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: u64,
    pub agent_user_id: String,
    pub user_contact_id: u64,
    pub local_display_name: String,
    pub profile: LocalProfile,
    pub active_user: bool,
    // view_pwd_hash: String, // Declared in the typescript API, but not sent by server
    pub show_ntfs: bool,
    #[serde(flatten)]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user: User,
    pub unread_count: u64,
    #[serde(flatten)]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub contact_id: u64,
    pub local_display_name: String,
    // profile: Profile,
    // active_conn: Connection,
    pub via_group: Option<u64>,
    // created_at: Date,
    #[serde(flatten)]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[serde(tag = "type")]
pub enum ChatInfo {
    Direct {
        contact: Contact,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    Group {
        group: GroupInfo,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ContactRequest {
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    #[serde(untagged)]
    Unknown(JsonValue),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupInfo {
    pub group_id: u64,
    pub local_display_name: String,
    pub group_profile: GroupProfile,
    pub membership: GroupMember,
    // pub created_at: Date, // TODO: Pick date type
    #[serde(flatten)]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupProfile {
    pub display_name: String,
    pub full_name: String,
    pub image: Option<String>,
    #[serde(flatten)]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub group_member_id: u64,
    pub member_id: String,
    pub member_role: GroupMemberRole,
    pub local_display_name: String,
    pub member_profile: Profile,
    pub member_contact_id: Option<u64>,
    pub active_conn: Option<Connection>,
    #[serde(flatten)]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum GroupMemberRole {
    Member,
    Admin,
    Owner,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    display_name: String,
    full_name: String,
    image: Option<String>,
    contact_link: Option<String>,
    #[serde(flatten)]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatItem {
    // chat_dir: CIDirection,
    #[serde(flatten)]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub chat_info: ChatInfo,
    // chat_items: Vec<ChatItem>,
    // chat_stats: ChatStats,
    #[serde(flatten)]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserContactLink {
    pub conn_req_contact: String,
    pub auto_accept: Option<AutoAccept>,
    #[serde(flatten)]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AutoAccept {
    pub accept_incognito: bool,
    pub auto_reply: Option<MsgContent>,
    #[serde(flatten)]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum MsgContent {
    Text {
        text: String,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    Link {
        text: String,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    Image {
        image: String, // Base64 string
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    File {
        text: String,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    Unknown(JsonValue),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[serde(tag = "type")]
pub enum ChatError {
    Error {
        error_type: ChatErrorType,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ErrorAgent {
        agent_error: JsonValue,
        #[serde(flatten)]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ErrorStore {
        store_error: JsonValue,
    },
    Unknown(JsonValue),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ChatErrorType {
    NoActiveUser,
    ActiveUserExists,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub conn_id: u64,
}
