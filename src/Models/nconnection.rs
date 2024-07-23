use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream, connect_async, tungstenite::Error };
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
                info!("Connected to {}", uri);
                return Ok(NConnection{ uri, nwebsocket })
                },
            Err(e) => Err(e),
        }
    }
}