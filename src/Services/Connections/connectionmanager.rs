pub mod connection_manager{
    pub async fn open_connection(uri: &str){
        info!("Connecting to {}", url);
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");
    }
}