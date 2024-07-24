use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Error, MaybeTlsStream, WebSocketStream};

pub struct NConnection {
    pub url: String,
    pub handle: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl NConnection {
    pub async fn new_nconnection(uri: String) -> Result<NConnection, Error> {
        match connect_async(&uri).await {
            Err(e) => {
                eprintln!("Could not connect to {} due to error: {}", &uri, e);
                return Err(e);
            }
            Ok((nwebsocket, _)) => {
                println!("Connected to {}", &uri);
                return Ok(NConnection {
                    url: uri,
                    handle: nwebsocket,
                });
            }
        }
    }
}
