use reqwest::Client;

pub async fn send_data_request(worker_url: &str, data: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post(worker_url)
        .body(data.to_owned())
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}