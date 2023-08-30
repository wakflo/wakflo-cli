use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct LoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub(crate) struct WakfloConfig {
    pub auth: Option<LoginResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct WakfloUser {
    pub email: String,
    pub name: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PluginCategory {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct WakfloResponse<T> {
    pub data: T,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
)]
pub(crate) enum PluginLanguage {
    Rust,
    Typescript,
    Javascript,
    Golang,
    Zig,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct WakfloExtension {
    pub plugin: WakfloPlugin,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct WakfloPlugin {
    pub name: String,
    pub version: String,
    pub description: String,
    pub language: PluginLanguage,
}
