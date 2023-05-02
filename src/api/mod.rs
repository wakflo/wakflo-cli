pub(crate) mod auth;
pub(crate) mod category;

use crate::api::auth::WakfloAuthApi;
use crate::api::category::WakfloCategoryApi;
use std::time::Duration;

pub struct WakfloApi {
    #[allow(unused)]
    base_url: String,
    #[allow(unused)]
    client: ureq::Agent,
    pub auth: WakfloAuthApi,
    pub category: WakfloCategoryApi,
}

impl WakfloApi {
    pub fn new() -> WakfloApi {
        let base_url = "https://dev-api.wakflo.io/v1".to_owned();
        let client: ureq::Agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();

        WakfloApi {
            base_url: base_url.clone(),
            client: client.clone(),
            auth: WakfloAuthApi::new(base_url.clone(), client.clone()),
            category: WakfloCategoryApi::new(base_url, client),
        }
    }
}

pub fn make_api() -> WakfloApi {
    WakfloApi::new()
}
