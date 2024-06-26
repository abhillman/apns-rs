use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Devices {
    pub(crate) device: Vec<Device>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Device {
    pub(crate) token: String,
}

impl Device {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
