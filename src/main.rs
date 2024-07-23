use log::LevelFilter;
use env_logger;
pub mod models;
pub mod enums;
pub mod services;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let uris = vec![String::from("wss://nostr.wine"), String::from("wss://relay.damus.io")];
    let connectionmanager = &models::nconnectionmanager::NConnectionManager;
    connectionmanager.connect_all(&uris).await;
    connectionmanager.subscribe(&uris).await;
}