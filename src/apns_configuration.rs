use isahc::{Body, RequestExt};
use serde::{Deserialize, Serialize};
use std::env;
use std::env::VarError;
use std::path::Path;

use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

use crate::apns_auth_token::ApnsAuthorization;
use crate::apns_body::ApnsBody;
use crate::common::Result;
use crate::devices::{Device, Devices};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ApnsConfiguration {
    topic: String,
    apns_authorization: ApnsAuthorization,
    apns_host: String,
    development: Devices,
}

impl ApnsConfiguration {
    const DEFAULT_PATH: &'static str = ".config/apns-rs/apns.toml";

    fn full_default_path() -> String {
        let key = "HOME";
        let home = match env::var_os(key) {
            Some(val) => val.to_str().unwrap().to_string(),
            None => panic!("$HOME is unset."),
        };
        format!("{}/{}", home, Self::DEFAULT_PATH)
    }

    pub(crate) fn load_default() -> Result<Self> {
        let path = match env::var("APNS_TOML") {
            Ok(val) => val,
            Err(e) => match e {
                VarError::NotPresent => {
                    let fdp = Self::full_default_path();
                    if Path::new(&fdp).try_exists()? {
                        fdp
                    } else {
                        return Err(Box::from(format!(
                            "Either APNS_TOML unset or ~/{} does not exist.",
                            Self::DEFAULT_PATH
                        )));
                    }
                }
                VarError::NotUnicode(_) => return Err(Box::from(e)),
            },
        };

        log::info!("Reading APNS configuration from {}", path);
        Self::load(path.as_ref())
    }

    pub(crate) fn load(path: &str) -> Result<Self> {
        let txt = std::fs::read_to_string(path)?;
        Ok(toml::from_str(txt.as_ref())?)
    }

    pub(crate) fn send_notifications<'a>(&'a self, apns_body: &'a ApnsBody) -> Result<()> {
        for device in &self.development.device {
            let auth_token = self.apns_authorization.encode()?;
            let url = format!("https://{}/3/device/{}", self.apns_host, device.token);

            log::info!("Sending message to {0}", device.token);
            isahc::Request::post(url)
                .header("apns-topic", self.topic.to_string())
                .header("apns-push-type", "alert")
                .header("authorization", format!("bearer {auth_token}"))
                .body(apns_body)?
                .send()?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn random() -> Self {
        let mut rng = StdRng::seed_from_u64(42);
        pub fn rand_id(rng: &mut impl RngCore, len: usize) -> String {
            let mut id: Vec<char> = vec![];
            for _ in 0..len {
                let range = match rng.gen_bool(0.5) {
                    true => b'A'..=b'Z',
                    false => b'0'..=b'9',
                };
                id.push(rng.gen_range(range).into())
            }
            id.iter().collect()
        }

        let team_id = rand_id(&mut rng, 10);

        Self {
            topic: "fun.aryeh.insanelygreat".into(),
            apns_authorization: ApnsAuthorization {
                auth_key_id: rand_id(&mut rng, 10),
                auth_key_path: format!("/secrets/AuthKey_{}.p8", team_id),
                team_id,
            },
            apns_host: "api.sandbox.push.apple.com:443".to_string(),
            development: Devices {
                device: vec![
                    Device::new(rand_id(&mut rng, 64)),
                    Device::new(rand_id(&mut rng, 64)),
                    Device::new(rand_id(&mut rng, 64)),
                ],
            },
        }
    }
}

impl From<&ApnsBody> for Body {
    fn from(apns_body: &ApnsBody) -> Self {
        Body::from(serde_json::to_string(&apns_body).unwrap())
    }
}
