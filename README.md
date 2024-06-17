# `apns-rs`

Simple CLI for delivering messages via [`APNS`](https://en.wikipedia.org/wiki/Apple_Push_Notification_service).

## Usage

```sh
$ apns-rs a b c
[2024-06-17T00:35:31Z INFO  apns_rs::apns_configuration] Reading APNS configuration from /home/a_user/.config/apns-rs/apns.toml
```

![screenshot.jpg](screenshot.jpg)

## Setup

```sh
$ cargo install --path .
$ cat >> $HOME/.config/apns-rs/apns.toml
topic = "fun.aryeh.exceptionalapp"
apns_host = "api.sandbox.push.apple.com:443"

[apns_authorization]
auth_key_id = "<AUTH_KEY_ID>"
team_id = "<TEAM_ID>"
p8_contents = """
-----BEGIN PRIVATE KEY-----
<PRIVATE_KEY>
-----END PRIVATE KEY-----
"""

[[development.device]]
token = "<DEVICE_TOKEN>"

[[development.device]]
token = "<ANOTHER_DEVICE_TOKEN>"
```

## Disclaimer

For demonstration purposes only. Not recommended for any use-case.

## License

The source code for the site is licensed under the MIT license, which you can find in the LICENSE file.