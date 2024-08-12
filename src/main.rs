pub mod enums;
pub mod models;
pub mod services;
use models::nconnectionmanager::NConnectionManager;
use yew::prelude::*;
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let connectionmanager = NConnectionManager::init();
    loop {
        let connections = connectionmanager.clone();
        let mut input_string = String::new();
        println!("Type a Nostr websocket url to connect to: ");
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
        println!("Connecting to {}", url);
        connections.connect_new(url).await;
    }
    html! { "hello world" }
}
