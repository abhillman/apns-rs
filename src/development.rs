use serde::{Deserialize, Serialize};
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Development {
    pub(crate) device_tokens: Vec<String>,
}

impl Development {
    pub fn from_toml(path: &str) -> Result<Self> {
        let txt = std::fs::read_to_string(path)?;

        #[derive(Serialize, Deserialize)]
        struct DevelopmentConfig {
            development: Development,
        }

        let development_config: DevelopmentConfig = toml::from_str(txt.as_ref())?;
        Ok(development_config.development)
    }

    pub fn from_toml_() -> Result<Self> {
        Self::from_toml("./apns.toml")
    }
}
