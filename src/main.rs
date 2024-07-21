use tokio_tungstenite::{ connect_async, tungstenite::protocol::Message };
use serde_json::Value;
use log::{ info, error, LevelFilter };
use futures_util::{SinkExt, StreamExt};
use env_logger;
mod models;
mod enums;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let url = "wss://nostr.wine";
    info!("Connecting to {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected");
    let (mut write, mut read) = ws_stream.split();
    //let nmessage: NMessage = 
    write.send(Message::Text("[\"REQ\",\"Event_ef872131-1e88-4183-8d74-b95c59d05058\",{\"ids\":[\"605b370290b863935b5cd36c06ec9ff989b8fd02bfb3c49172cabb987920b16f\"],\"kinds\":[1],\"since\":1089888389,\"until\":1721126789,\"limit\":1},{\"kinds\":[1],\"#e\":[\"605b370290b863935b5cd36c06ec9ff989b8fd02bfb3c49172cabb987920b16f\"],\"since\":1089888389,\"until\":1721126789,\"limit\":5}]".to_string()+"\n")).await.unwrap();
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => handle_message(&text),
            Ok(_) => (), // Ignore non-Text messages
            Err(e) => error!("Error: {}", e),
        }
    }
}



fn handle_message(text: &str) {
    match serde_json::from_str::<Value>(&text) {
        Ok(data) => {
            match (data[0].as_str(), data[1].as_str()) {
                (Some(eventtype), Some(subid)) => {
                    info!("New message received: Type: {}, SubId: {}", eventtype, subid);
                }
                _ => error!("Incorrect message structure"),
            }
        }
        Err(_) => error!("Failed to parse message: {}", text),
    }
}