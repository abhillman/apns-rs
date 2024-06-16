use std::error;
use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Serialize, Deserialize)]
struct Claims {
    iss: String,
    iat: u64,
}

impl Claims {
    fn new(team_id: String, epoch_time: u64) -> Self {
        Self {
            iss: team_id,
            iat: epoch_time,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ApnsAuthToken {
    auth_key_id: String,
    p8_contents: String,
    team_id: String,
}

impl ApnsAuthToken {
    pub fn from_toml_() -> Result<Self> {
        Self::from_toml("./apns.toml")
    }

    pub fn from_toml(path: &str) -> Result<Self> {
        #[derive(Deserialize)]
        struct ApnsConfig {
            apns: ApnsAuthToken,
        }

        let txt = std::fs::read_to_string(path)?;
        let apns_config: ApnsConfig = toml::from_str(txt.as_ref())?;
        Ok(apns_config.apns)
    }

    fn encoding_key(p8_contents: &String) -> Result<EncodingKey> {
        Ok(EncodingKey::from_ec_pem(p8_contents.as_ref())?)
    }

    fn header(auth_key_id: &str) -> Header {
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(auth_key_id.to_owned());
        header.typ = None;
        header
    }

    pub(crate) fn encode(&self) -> Result<String> {
        let header = Self::header(&self.auth_key_id);
        let encoding_key = Self::encoding_key(&self.p8_contents);
        let epoch_time: u64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let claims = Claims::new(self.team_id.clone(), epoch_time);
        Ok(encode(&header, &claims, &encoding_key?)?)
    }
}
