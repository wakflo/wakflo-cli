use serde::{Serialize, Deserialize};
use crate::utils::dir_files::get_wakflo_config;
use crate::utils::types::{PluginCategory, WakfloResponse, WakfloUser};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct WhoAmIResponse {
    pub me: WakfloUser,
}

pub struct WakfloCategoryApi {
    base_url: String,
    client: ureq::Agent,
}

impl WakfloCategoryApi {
    pub fn new(base_url: String, client: ureq::Agent) -> WakfloCategoryApi {
        WakfloCategoryApi {
            base_url,
            client,
        }
    }

    pub(crate) fn list(&self) -> anyhow::Result<Vec<PluginCategory>> {
        let url = format!("{}{}", self.base_url, "/categories");
        let mut req = self.client.get(url.as_str());

        let config = get_wakflo_config()?;
        if config.auth.is_some() {
            req = req.set("Authorization", format!("Bearer {}", config.auth.unwrap().access_token).as_str());
        }

        let response = req.call()?;
        let json = response.into_json::<WakfloResponse<Vec<PluginCategory>>>()?;
        Ok(json.data)
    }
}