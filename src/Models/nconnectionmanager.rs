use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::{ connect_async, tungstenite::protocol::Message };
use log::{ info, error, LevelFilter };
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;

use super::nconnection::NConnection;

pub struct NConnectionManager<'a> {
    pub uris: &'a Vec<String>,
    pub nwebsockets: Vec<NConnection<'a>>
}

impl<'a> NConnectionManager<'a> {
    pub async fn connect_all(uris: &'a Vec<String>) -> Self{
        let nwebsocketbucket = Vec::<NConnection>::with_capacity(128);
        let mut connectionmanager = NConnectionManager{ uris, nwebsockets: nwebsocketbucket };
        for uri in uris {
            match NConnection::new(&uri).await {
                Ok(connection) => { 
                    connectionmanager.nwebsockets.push(connection);
                    },
                Err(e) => {
                    info!("Could not connect to {} due to error: {}", uri, e);
                },
            }
        }
        connectionmanager
    }
    pub async fn fetch_by_filter(filter: String, connectionmanager: &NConnectionManager<'a>){
        for websocket in connectionmanager.nwebsockets {
            let (mut write, mut read) = websocket.nwebsocket.split();
            write.send(Message::Text(filter)).await.unwrap();
            while let Some(message) = read.next().await {
                match message {
                    Ok(Message::Text(text)) => {

                    },
                    Ok(_) => (), // Ignore non-Text messages
                    Err(e) => error!("Error: {}", e),
                }
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
}