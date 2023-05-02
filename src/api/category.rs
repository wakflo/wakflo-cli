<<<<<<< HEAD
use crate::utils::dir_files::get_wakflo_config;
use crate::utils::types::{PluginCategory, WakfloResponse, WakfloUser};
use serde::{Deserialize, Serialize};
=======
use serde::{Serialize, Deserialize};
use crate::utils::dir_files::get_wakflo_config;
use crate::utils::types::{PluginCategory, WakfloResponse, WakfloUser};
>>>>>>> 85173fa (feat: first commit)

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
<<<<<<< HEAD
        WakfloCategoryApi { base_url, client }
=======
        WakfloCategoryApi {
            base_url,
            client,
        }
>>>>>>> 85173fa (feat: first commit)
    }

    pub(crate) fn list(&self) -> anyhow::Result<Vec<PluginCategory>> {
        let url = format!("{}{}", self.base_url, "/categories");
        let mut req = self.client.get(url.as_str());

        let config = get_wakflo_config()?;
        if config.auth.is_some() {
<<<<<<< HEAD
            req = req.set(
                "Authorization",
                format!(
                    "Bearer {}",
                    config.auth.expect("missing auth api").access_token
                )
                .as_str(),
            );
=======
            req = req.set("Authorization", format!("Bearer {}", config.auth.unwrap().access_token).as_str());
>>>>>>> 85173fa (feat: first commit)
        }

        let response = req.call()?;
        let json = response.into_json::<WakfloResponse<Vec<PluginCategory>>>()?;
        Ok(json.data)
    }
<<<<<<< HEAD
}
=======
}
>>>>>>> 85173fa (feat: first commit)
