use openapi::apis::{configuration::Configuration, default_api as twilio_api};
use std::env;
use tide::prelude::*;
use tide::{Body, Request, Response};

async fn get_push_it(mut _req: Request<()>) -> tide::Result {
    Ok(Response::builder(200)
        .body(Body::from_file("static/saltandpepper-pushit.mp3").await?)
        .build())
}

async fn post_twilio_call_callback(mut _req: Request<()>) -> tide::Result {
    println!("Twilio call callback received!");

    let payload = r#"
<?xml version="1.0" encoding="UTF-8"?>
<Response>
    <Play><a href="http://ocrmirror.org/files/music/remixes/Street_Fighter_2_Guile%27s_Theme_Goes_with_Metal_OC_ReMix.mp3">http://ocrmirror.org/files/music/remixes/Street_Fighter_2_Guile's_Theme_Goes_with_Metal_OC_ReMix.mp3</a></Play>
</Response>"#.to_string();

    Ok(Response::builder(200)
        .body(payload)
        .header("content-type", "text/xml")
        .build())
}

#[tokio::main]
async fn main() {
    let mut app = tide::new();
    app.at("/twilio/call/callback")
        .post(post_twilio_call_callback);
    app.at("/static/push-it.mp3").get(get_push_it);
    let mut listener = app.bind("127.0.0.1:8080").await.expect("could not listen");
    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }
    tokio::spawn(async move {
        listener
            .accept()
            .await
            .expect("could not accept connections");
    });

    let account_sid = env::var("TWILIO_ACCOUNT_SID").expect("Failed to parse Account SID");
    let api_key = env::var("TWILIO_API_KEY").expect("Failed to parse API Key");
    let api_key_secret = env::var("TWILIO_API_KEY_SECRET").expect("Failed to parse API Key Secret");
    let from = env::var("TWILIO_PHONE_NUMBER").expect("Failed to parse 'from' number");
    let to = env::var("TO_NUMBER").expect("Failed to parse 'to' number");

    let mut twilio_config = Configuration::default();
    twilio_config.basic_auth = Some((api_key, Some(api_key_secret)));

    //    // Asynchronously send the message "Ahoy, Rustacean! 🦀" to the `to` number from your Twilio phone number.
    //    let message = twilio_api::create_message(
    //        &twilio_config,
    //        &account_sid,
    //        &to,
    //        None,
    //        None,
    //        None,
    //        Some("Ahoy, Rustacean! 🦀"),
    //        None,
    //        None,
    //        Some(&from),
    //        None,
    //        None,
    //        None,
    //        None,
    //        None,
    //        None,
    //        None,
    //        None,
    //        None,
    //        None,
    //        None,
    //    )
    //    .await;
    //
    //    let result = match message {
    //        Ok(result) => result,
    //        Err(error) => panic!("Something went wrong, {:?}", error),
    //    };

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1));
    }

    println!("Hello, world!");
}
