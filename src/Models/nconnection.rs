use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream, connect_async, tungstenite::Error, tungstenite::protocol::Message};
use serde_json::Value;
use log::info;

pub struct NConnection<'a> {
    pub uri: &'a String,
    pub nwebsocket: WebSocketStream<MaybeTlsStream<TcpStream>>
}

impl<'a> NConnection<'a> {
    pub async fn new(uri: &'a String) -> Result<Self, Error>{
        info!("Connecting to {}", uri);
        match connect_async(uri).await {
            Ok((nwebsocket, _)) => { 
                info!("Connected {}", uri);
                return Ok(NConnection{ uri, nwebsocket })
                },
            Err(e) => Err(e),
        }
    }

    pub async fn fetch_by_filter() {
        let uri = String::from("wss://nostr.wine");
        let newconnection = NConnection::new(&uri).await;
    }
}