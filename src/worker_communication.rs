// src/worker_communication.rs
use reqwest::Client;

pub async fn send_data_request(worker_url: &str, data: &str) -> String {
    let client = Client::new();
    let response = client
        .post(worker_url)
        .body(data.to_owned())
        .send()
        .await
        .unwrap();
    response.text().await.unwrap()
}