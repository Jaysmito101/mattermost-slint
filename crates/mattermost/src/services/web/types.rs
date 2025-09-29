use serde::{Deserialize, Serialize};

/// https://developers.mattermost.com/api-documentation/#/operations/Login
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LoginData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub login_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldap_only: Option<bool>,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NotifyProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop_sound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_keys: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Timezone {
    #[serde(rename = "useAutomaticTimezone")]
    pub use_automatic_timezone: Option<String>,
    #[serde(rename = "manualTimezone")]
    pub manual_timezone: Option<String>,
    #[serde(rename = "automaticTimezone")]
    pub automatic_timezone: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct User {
    pub id: String,
    pub create_at: i64,
    pub update_at: i64,
    pub delete_at: i64,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    pub email: String,
    pub email_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_service: Option<String>,
    pub roles: String,
    pub locale: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_props: Option<NotifyProps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_password_update: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_picture_update: Option<i64>,
    pub failed_attempts: i32,
    pub mfa_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<Timezone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_create_at: Option<i64>,
}

#[derive(Clone, Debug, Default)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

pub enum WebApiCommand {
    SetConfig(String, String, Box<dyn FnOnce() + Send>),
    UserLogin(LoginData, Box<dyn FnOnce(Result<LoginResponse, crate::Error>) + Send>),
}
