extern crate core;

use crate::apns_body::{Alert, AlertWrapper, ApnsBody};
use crate::apns_configuration::ApnsConfiguration;

mod apns_auth_token;
mod apns_body;
mod apns_configuration;
mod common;
mod devices;

fn main() -> crate::common::Result<()> {
    let title: String;
    if let Some(title_) = std::env::args().collect::<Vec<_>>().get(1) {
        title = title_.clone();
    } else {
        title = "zz".to_string();
    }

    let subtitle: String;
    if let Some(subtitle_) = std::env::args().collect::<Vec<_>>().get(2) {
        subtitle = subtitle_.clone();
    } else {
        subtitle = "love u".to_string();
    }

    let body: String;
    if let Some(body_) = std::env::args().collect::<Vec<_>>().get(3) {
        body = body_.clone();
    } else {
        body = "u r kewl v kewl".to_string();
    }

    let ac = ApnsConfiguration::load_default();
    let apns_body = ApnsBody {
        aps: AlertWrapper {
            alert: Alert {
                title: title.to_string(),
                subtitle: subtitle.to_string(),
                body: body.to_string(),
            },
        },
    };

    ac.unwrap().send_notifications(&apns_body)
}
