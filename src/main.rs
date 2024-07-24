pub mod enums;
pub mod models;
pub mod services;
use models::nconnectionmanager::NConnectionManager;

#[tokio::main]

async fn main() {
    use std::io::{stdin, stdout, Write};
    let mut connectionmanager = NConnectionManager::init();
    loop {
        print!(
            "Currently connected to {} relays.",
            &connectionmanager.connections.len()
        );
        for connection in &connectionmanager.connections {
            print!("{}", connection.url);
        }
        let mut input_string = String::new();
        print!("Type a Nostr websocket url to connect to: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input_string)
            .expect("Incorrect string entered!");
        if let Some('\n') = input_string.chars().next_back() {
            input_string.pop();
        }
        if let Some('\r') = input_string.chars().next_back() {
            input_string.pop();
        }
        let url = String::from(input_string);
        print!("Connecting to {}", url);
        connectionmanager.connect_new(url).await;
    }
}
