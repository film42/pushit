use openapi::apis::{configuration::Configuration, default_api as twilio_api};
use std::env;

#[tokio::main]
async fn main() {
    let account_sid = env::var("TWILIO_ACCOUNT_SID").expect("Failed to parse Account SID");
    let api_key = env::var("TWILIO_API_KEY").expect("Failed to parse API Key");
    let api_key_secret = env::var("TWILIO_API_KEY_SECRET").expect("Failed to parse API Key Secret");
    let from = env::var("TWILIO_PHONE_NUMBER").expect("Failed to parse 'from' number");
    let to = env::var("TO_NUMBER").expect("Failed to parse 'to' number");

    let mut twilio_config = Configuration::default();
    twilio_config.basic_auth = Some((api_key, Some(api_key_secret)));

    // Asynchronously send the message "Ahoy, Rustacean! 🦀" to the `to` number from your Twilio phone number.
    let message = twilio_api::create_message(
        &twilio_config,
        &account_sid,
        &to,
        None,
        None,
        None,
        Some("Ahoy, Rustacean! 🦀"),
        None,
        None,
        Some(&from),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    let result = match message {
        Ok(result) => result,
        Err(error) => panic!("Something went wrong, {:?}", error),
    };

    println!("Hello, world!");
}
