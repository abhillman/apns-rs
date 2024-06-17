# `apns-rs`

Simple CLI for delivering messages via [`APNS`](https://en.wikipedia.org/wiki/Apple_Push_Notification_service).

## Usage

```sh
$ apns-rs a b c # apns-rs <title> <subtitle> <body>
[2024-06-17T06:42:13Z INFO  apns_rs::apns_configuration] Reading APNS configuration from /home/a_user/.config/apns-rs/apns.toml
[2024-06-17T06:42:13Z INFO  apns_rs::apns_configuration] Sending message to F96230A6G1A99792Z2WM9463H6S62MC0V3H730S9DO8IG8T292N1EP25T508N9Y8
[2024-06-17T06:42:13Z INFO  apns_rs::apns_configuration] Sending message to CC2DC8PC526CC4F5GMQA15RR2LE2OIX4IWV910OZ4FA76TB9186D2FO85KVT6W73
[2024-06-17T06:42:13Z INFO  apns_rs::apns_configuration] Sending message to J64V4BNWIH2A454W0M2349N1H731B317OW79I77DN6X79WY0T03B74852QCS56Z5
```

![screenshot.jpg](screenshot.jpg)

## Setup

1. Write a `apns.toml` file to `~/.config/apns-rs/apns.toml`; here is an example:

```toml
topic = "fun.aryeh.insanelygreat"
apns_host = "api.sandbox.push.apple.com:443"

[apns_authorization]
auth_key_id = "QNRVZBHUNR"
auth_key_path = "/secrets/AuthKey_26Z411V5S5.p8"
team_id = "26Z411V5S5"

[[development.device]]
token = "F96230A6G1A99792Z2WM9463H6S62MC0V3H730S9DO8IG8T292N1EP25T508N9Y8"

[[development.device]]
token = "CC2DC8PC526CC4F5GMQA15RR2LE2OIX4IWV910OZ4FA76TB9186D2FO85KVT6W73"

[[development.device]]
token = "J64V4BNWIH2A454W0M2349N1H731B317OW79I77DN6X79WY0T03B74852QCS56Z5"
```

>**Tip**: you can use `APNS_TOML` to specify an alternative path

2. Run `cargo install --path .`
3. Send a notification with `apns-rs <title> <subtitle> <body>`

## Disclaimer

For demonstration purposes only. Not recommended for any use-case.

## License

The source code for the site is licensed under the MIT license, which you can find in the [`LICENSE`](LICENSE) file.
