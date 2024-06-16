extern crate core;

use std::fs::read_to_string;

use isahc::RequestExt;
use toml::Value;

use crate::apns_auth_token::ApnsAuthToken;
use crate::apns_data::{Alert, AlertWrapper, ApnsNotification};
use crate::development::Development;

mod apns_auth_token;
mod apns_data;
mod development;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

fn main() -> Result {
    let apns_toml: Value = toml::from_str(read_to_string("apns.toml")?.as_ref())?;
    let topic = apns_toml.get("topic").unwrap().as_str().unwrap();
    let apns_host = apns_toml.get("apns_host").unwrap().as_str().unwrap();

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

    for device_token in Development::from_toml_()?.device_tokens {
        let url = format!("https://{apns_host}/3/device/{device_token}");

        let notification = ApnsNotification {
            aps: AlertWrapper {
                alert: Alert {
                    title: title.to_string(),
                    subtitle: subtitle.to_string(),
                    body: body.to_string(),
                },
            },
        };
        let body = serde_json::to_string(&notification)?;

        let auth_token = ApnsAuthToken::from_toml_()?.encode()?;
        let response = isahc::Request::post(url)
            .header("apns-topic", topic)
            .header("apns-push-type", "alert")
            .header("authorization", format!("bearer {auth_token}"))
            .body(body)?
            .send()?;
        println!("{response:#?}");
    }

    Ok(())
}
