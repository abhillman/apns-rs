use isahc::{Body, RequestExt};
use serde::Deserialize;

use crate::apns_auth_token::ApnsAuthorization;
use crate::apns_body::ApnsBody;
use crate::common::Result;
use crate::devices::Devices;

#[derive(Debug, Deserialize)]
pub(crate) struct ApnsConfiguration {
    topic: String,
    apns_authorization: ApnsAuthorization,
    apns_host: String,
    development: Devices,
}

impl ApnsConfiguration {
    pub(crate) fn load_default() -> Result<Self> {
        Self::load("./apns.toml")
    }

    pub(crate) fn load(path: &str) -> Result<Self> {
        let txt = std::fs::read_to_string(path)?;
        Ok(toml::from_str(txt.as_ref())?)
    }

    pub(crate) fn send_notifications<'a>(&'a self, apns_body: &'a ApnsBody) -> Result<()> {
        for device in &self.development.device {
            let auth_token = self.apns_authorization.encode()?;
            let url = format!("https://{}/3/device/{}", self.apns_host, device.token);
            isahc::Request::post(url)
                .header("apns-topic", self.topic.to_string())
                .header("apns-push-type", "alert")
                .header("authorization", format!("bearer {auth_token}"))
                .body(apns_body)?
                .send()?;
        }
        Ok(())
    }
}

impl From<&ApnsBody> for Body {
    fn from(apns_body: &ApnsBody) -> Self {
        Body::from(serde_json::to_string(&apns_body).unwrap())
    }
}
