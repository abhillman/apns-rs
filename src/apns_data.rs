use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ApnsNotification {
    pub aps: AlertWrapper,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AlertWrapper {
    pub alert: Alert,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Alert {
    pub title: String,
    pub subtitle: String,
    pub body: String,
}
