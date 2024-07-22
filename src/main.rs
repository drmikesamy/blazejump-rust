use log::LevelFilter;
use env_logger;
pub mod models;
pub mod enums;
pub mod services;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    models::nconnection::NConnection::fetch_by_filter().await;
}