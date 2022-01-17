use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::apps::v1::Deployment;
use kube::{api::ListParams, client::Client, core::WatchEvent, Api};
use openapi::apis::{configuration::Configuration, default_api as twilio_api};
use std::env;
use tide::prelude::*;
use tide::{Body, Request, Response};
use tokio::sync::mpsc::{Receiver as MpscReceiver, Sender as MpscSender};

struct TwilioCaller {
    configuration: Configuration,
    account_sid: String,
    twilio_number: String,
    to_number: String,
    call_callback_url: String,
}

impl TwilioCaller {
    async fn create_call(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Woof look at this boilerplate code.
        let call = twilio_api::create_call(
            &self.configuration,
            &self.account_sid,
            &self.twilio_number,
            &self.to_number,
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
            Some(&self.call_callback_url),
        )
        .await?;

        println!("Call made to twilio: {:?}", call);

        Ok(())
    }
}

async fn get_push_it(mut _req: Request<WebContext>) -> tide::Result {
    Ok(Response::builder(200)
        .body(Body::from_file("static/saltandpepper-pushit.mp3").await?)
        .build())
}

async fn post_twilio_call_callback(req: Request<WebContext>) -> tide::Result {
    let payload = req.state().payload_for_caller();
    println!(
        "Twilio call callback POST request received! Responding with: {}",
        &payload
    );

    Ok(Response::builder(200)
        .body(payload)
        .header("Content-Type", "text/xml")
        .build())
}

#[derive(Clone)]
struct WebContext {
    base_url: String,
}

impl WebContext {
    fn payload_for_caller(&self) -> String {
        let song_url = format!("{}/static/push-it.mp3", &self.base_url);
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?><Response><Play>{}</Play></Response>"#,
            &song_url,
        )
        .to_string()
    }
}

async fn debounce_events_and_make_twilio_call(
    twilio: TwilioCaller,
    mut receiver: MpscReceiver<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // The receiver was closed.
        if receiver.recv().await.is_none() {
            return Ok(());
        }
        println!("Received event. Debouncing...");

        // Sleep for 10 seconds to debounce.
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        // Throw away all the other events since we're about to make
        // a call.
        while let Ok(_) = receiver.try_recv() { /* ignore extra events */ }

        // Push it!
        println!("Triggering a call to twilio.");
        twilio.create_call().await?;
    }
}

async fn watch_for_kubernetes_deployment_changes(
    client: Client,
    sender: MpscSender<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let deployment_api: Api<Deployment> = Api::all(client);
    let params = ListParams::default();

    let resource_version = deployment_api
        .list(&params)
        .await?
        .metadata
        .resource_version
        .expect("invalid call");

    let mut stream = deployment_api
        .watch(&params, &resource_version)
        .await?
        .boxed();

    println!("Watching for deployment changes...");

    while let Some(event) = stream.try_next().await? {
        match event {
            WatchEvent::Added(deployment) | WatchEvent::Modified(deployment) => {
                if let Some("pushit") = deployment.metadata.name.as_ref().map(|s| &s[..]) {
                    println!("Detected a deployment change about myself (pushit), skipping...");
                    continue;
                }

                // Push it!
                println!("The kube api notified of a deployment event!!!");
                sender.send(1337).await?;
                println!("Watcher informed the twilio caller.");
            }
            _ => { /* ignore */ }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configs from env
    let account_sid = env::var("TWILIO_ACCOUNT_SID").expect("Failed to parse Account SID");
    let api_key = env::var("TWILIO_API_KEY").expect("Failed to parse API Key");
    let api_key_secret = env::var("TWILIO_API_KEY_SECRET").expect("Failed to parse API Key Secret");
    let twilio_number = env::var("TWILIO_PHONE_NUMBER").expect("Failed to parse 'from' number");
    let to_number = env::var("TO_NUMBER").expect("Failed to parse 'to' number");
    let base_url = env::var("APPLICATION_BASE_URL").expect("Failed to parse 'base url' value");

    // Configure the webserver that handles callbacks and static assets.
    let mut app = tide::with_state(WebContext {
        base_url: base_url.clone(),
    });
    app.at("/twilio/call/callback")
        .post(post_twilio_call_callback);
    app.at("/static/push-it.mp3").get(get_push_it);
    let mut listener = app.bind("0.0.0.0:8080").await.expect("could not listen");
    for info in listener.info().iter() {
        println!("Server listening on {}", info);
    }
    tokio::spawn(async move {
        listener
            .accept()
            .await
            .expect("could not accept connections");
    });

    // Configure Twilio
    let call_callback_url = format!("{}/twilio/call/callback", &base_url).to_string();
    println!("Twilio callback: {}", call_callback_url);
    let mut twilio_config = Configuration::default();
    twilio_config.basic_auth = Some((api_key, Some(api_key_secret)));
    let twilio_caller = TwilioCaller {
        configuration: twilio_config,
        account_sid,
        twilio_number,
        to_number,
        call_callback_url,
    };

    let (kube_events_sender, kube_events_receiver) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        debounce_events_and_make_twilio_call(twilio_caller, kube_events_receiver)
            .await
            .unwrap();
    });

    // Initialize the kube controller
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default().await?;
    // The api will cancel the watch after a while. Retry or abort.
    loop {
        watch_for_kubernetes_deployment_changes(client.clone(), kube_events_sender.clone())
            .await
            .expect("failed to watch");

        println!("Watcher finished, restarting...");
    }

    println!("Exiting...");

    Ok(())
}
