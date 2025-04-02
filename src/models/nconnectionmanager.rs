use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tokio_tungstenite::{connect_async, tungstenite::Error};

use crate::enums::kindenum::KindEnum;
#[derive(Clone)]
pub struct NConnectionManager {
    pub connections: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
}

impl NConnectionManager {
    pub fn init() -> Self {
        let db = Arc::new(Mutex::new(HashMap::new()));
        NConnectionManager { connections: db }
    }
    pub async fn connect_new(self, uri: String) {
        let theuri = uri.clone();
        self.connections.lock().unwrap().insert(
            uri,
            tokio::spawn(async move {
                match connect_async(theuri).await {
                    Err(e) => {
                        eprintln!("Could not connect due to error: {}", e);
                    }
                    Ok((nwebsocket, _)) => {
                        println!("Connected");
                    }
                }
            }),
        );
    }
    // pub async fn fetch_by_filter(filter: String, connectionmanager: &NConnectionManager<'a>){
    //     for websocket in connectionmanager.nwebsockets {
    //         let (mut write, mut read) = websocket.nwebsocket.split();
    //         write.send(Message::Text(filter)).await.unwrap();
    //         while let Some(message) = read.next().await {
    //             match message {
    //                 Ok(Message::Text(text)) => {

    //                 },
    //                 Ok(_) => (), // Ignore non-Text messages
    //                 Err(e) => error!("Error: {}", e),
    //             }
    //         }
    //     }
    // }
    // fn handle_message(text: &str) {
    //     match serde_json::from_str::<Value>(&text) {
    //         Ok(data) => {
    //             match (data[0].as_str(), data[1].as_str()) {
    //                 (Some(eventtype), Some(subid)) => {
    //                     info!("New message received: Type: {}, SubId: {}", eventtype, subid);
    //                 }
    //                 _ => error!("Incorrect message structure"),
    //             }
    //         }
    //         Err(_) => error!("Failed to parse message: {}", text),
    //     }
    // }
}
