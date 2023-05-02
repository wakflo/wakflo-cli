use crate::utils::dir_files::get_wakflo_config;
use crate::utils::types::{LoginResponse, WakfloUser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct WhoAmIResponse {
    pub me: WakfloUser,
}

pub struct WakfloAuthApi {
    base_url: String,
    client: ureq::Agent,
}

impl WakfloAuthApi {
    pub fn new(base_url: String, client: ureq::Agent) -> WakfloAuthApi {
        WakfloAuthApi { base_url, client }
    }

    pub(crate) fn login(
        &self,
        identifier: String,
        password: String,
    ) -> anyhow::Result<LoginResponse> {
        let url = format!("{}{}", self.base_url, "/auth/login");
        let body = LoginRequest {
            password,
            identifier,
        };

        let response = self.client.post(url.as_str()).send_json(body)?;
        let json = response.into_json::<LoginResponse>()?;
        Ok(json)
    }

    pub(crate) fn whoami(&self) -> anyhow::Result<WakfloUser> {
        let url = format!("{}{}", self.base_url, "/auth/whoami");
        let mut req = self.client.get(url.as_str());

        let config = get_wakflo_config()?;
        if config.auth.is_some() {
            req = req.set(
                "Authorization",
                format!(
                    "Bearer {}",
                    config.auth.expect("missing auth api").access_token
                )
                .as_str(),
            );
        }

        let response = req.call()?;
        let json = response.into_json::<WhoAmIResponse>()?;
        Ok(json.me)
    }
}
