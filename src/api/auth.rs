use crate::utils::dir_files::get_wakflo_config;
use crate::utils::types::{LoginResponse, WakfloUser};
use serde::{Deserialize, Serialize};

trait Block {
    fn wait(self) -> <Self as futures::Future>::Output
    where
        Self: Sized,
        Self: futures::Future,
    {
        futures::executor::block_on(self)
    }
}

impl<F, T> Block for F where F: futures::Future<Output = T> {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct WhoAmIResponse {
    pub me: WakfloUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CustomFlow {
    /// CreatedAt is a helper struct field for gobuffalo.pop.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// ExpiresAt is the time (UTC) when the flow expires. If the user still wishes to log in, a new flow has to be initiated.
    #[serde(rename = "expires_at")]
    pub expires_at: String,
    /// ID represents the flow's unique ID. When performing the login flow, this represents the id in the login UI's query parameter: http://<selfservice.flows.login.ui_url>/?flow=<flow_id>
    #[serde(rename = "id")]
    pub id: String,
    /// IssuedAt is the time (UTC) when the flow started.
    #[serde(rename = "issued_at")]
    pub issued_at: String,
    /// Ory OAuth 2.0 Login Challenge.  This value is set using the `login_challenge` query parameter of the registration and login endpoints. If set will cooperate with Ory OAuth2 and OpenID to act as an OAuth2 server / OpenID Provider.
    #[serde(
        rename = "oauth2_login_challenge",
        skip_serializing_if = "Option::is_none"
    )]
    pub oauth2_login_challenge: Option<String>,
    /// Refresh stores whether this login flow should enforce re-authentication.
    #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>,
    /// RequestURL is the initial URL that was requested from Ory Kratos. It can be used to forward information contained in the URL's path or query for example.
    #[serde(rename = "request_url")]
    pub request_url: String,
    /// ReturnTo contains the requested return_to URL.
    #[serde(rename = "return_to", skip_serializing_if = "Option::is_none")]
    pub return_to: Option<String>,
    /// SessionTokenExchangeCode holds the secret code that the client can use to retrieve a session token after the login flow has been completed. This is only set if the client has requested a session token exchange code, and if the flow is of type \"api\", and only on creating the login flow.
    #[serde(
        rename = "session_token_exchange_code",
        skip_serializing_if = "Option::is_none"
    )]
    pub session_token_exchange_code: Option<String>,
    /// The flow type can either be `api` or `browser`.
    #[serde(rename = "type")]
    pub _type: String,
}

pub struct WakfloAuthApi {
    base_url: String,
    client: ureq::Agent,
    ory_conf: ory_client::apis::configuration::Configuration,
}

impl WakfloAuthApi {
    pub fn new(base_url: String, client: ureq::Agent) -> WakfloAuthApi {
        let mut conf = ory_client::apis::configuration::Configuration::new();
        conf.base_path = "http://localhost:4433".to_owned();
        // conf.base_path = "http://api.wakflo.io/auth".to_owned();

        WakfloAuthApi {
            base_url,
            client,
            ory_conf: conf,
        }
    }

    pub(crate) fn login(
        &self,
        identifier: String,
        password: String,
    ) -> anyhow::Result<LoginResponse> {
        let local_var_uri_str = format!(
            "{}/self-service/login/api?return_session_token_exchange_code=true",
            &self.ory_conf.base_path
        );
        let flow_rsp = self.client.get(local_var_uri_str.as_str()).call()?;
        let flow = flow_rsp.into_json::<CustomFlow>()?;

        let rsp = ory_client::apis::frontend_api::update_login_flow(
            &self.ory_conf,
            flow.id.clone().as_str(),
            &ory_client::models::UpdateLoginFlowBody::UpdateLoginFlowWithPasswordMethod {
                csrf_token: None,
                identifier: identifier.clone(),
                password: password.clone(),
                password_identifier: None,
            },
            None,
            None,
        )
        .wait()
        .expect("");

        Ok(LoginResponse {
            access_token: rsp.session_token.expect("failed to get session"),
        })
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

        match req.call() {
            Ok(response) => {
                let json = response.into_json::<WhoAmIResponse>()?;
                Ok(json.me)
            }
            Err(e) => {
                anyhow::bail!(format!(
                    "{}",
                    e.to_string()
                        .replace("https://api.wakflo.io/v1/auth/whoami:", "")
                ))
            }
        }
    }
}
