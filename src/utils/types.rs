<<<<<<< HEAD
use serde::{Deserialize, Serialize};
=======
use serde::{Serialize, Deserialize};
>>>>>>> 85173fa (feat: first commit)

#[derive(Debug, Serialize, Deserialize)]
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
<<<<<<< HEAD
}
=======
}
>>>>>>> 85173fa (feat: first commit)
