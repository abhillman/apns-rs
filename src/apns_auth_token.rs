use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

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
pub struct ApnsAuthorization {
    auth_key_id: String,
    p8_contents: String,
    team_id: String,
}

impl ApnsAuthorization {
    fn encoding_key(p8_contents: &String) -> crate::common::Result<EncodingKey> {
        Ok(EncodingKey::from_ec_pem(p8_contents.as_ref())?)
    }

    fn header(auth_key_id: &str) -> Header {
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(auth_key_id.to_owned());
        header.typ = None;
        header
    }

    pub(crate) fn encode(&self) -> crate::common::Result<String> {
        let header = Self::header(&self.auth_key_id);
        let encoding_key = Self::encoding_key(&self.p8_contents);
        let epoch_time: u64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let claims = Claims::new(self.team_id.clone(), epoch_time);
        Ok(encode(&header, &claims, &encoding_key?)?)
    }
}
