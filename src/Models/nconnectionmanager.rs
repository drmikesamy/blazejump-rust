use tokio_tungstenite::tungstenite::Error;
use super::nconnection::NConnection;

pub struct NConnectionManager {
    pub connections: Vec<NConnection>
}

impl NConnectionManager {
    pub fn init() -> Self {
        NConnectionManager{ connections: Vec::with_capacity(1000) }
    }
    pub async fn connect_new(&mut self, uri: String) -> bool{
        match NConnection::new_nconnection(uri).await {
            Ok(nconnection) => {
                self.connections.push(nconnection);
                return true;
            }
            Err(e) => {
                return false;
            }
        }
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