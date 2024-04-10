// src/worker_communication.rs
use reqwest::Client;

pub async fn send_data_request(worker_url: &str, data: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post(worker_url)
        .body(data.to_owned())
        .send()
        .await?;

    if response.status().is_success() {
        println!("Request sent successfully to the Cloudflare Worker!");
        Ok(response.text().await?)
    } else {
        println!("Error sending request to the Cloudflare Worker: {:?}", response.status());
        Err(reqwest::Error::new(
            reqwest::StatusCode::from_u16(response.status().as_u16()).unwrap(),
            format!("Request failed with status: {}", response.status()),
        ))
    }
}