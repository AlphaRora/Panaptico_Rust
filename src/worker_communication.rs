// src/worker_communication.rs
use reqwest::Client;

pub async fn send_command_request(worker_url: &str) -> String {
    let client = Client::new();
    let response = client.get(worker_url).send().await.unwrap();
    response.text().await.unwrap()
}