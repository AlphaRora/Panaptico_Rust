use reqwest::{Client, Response};

pub async fn send_data_request(worker_url: &str, data: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();

    println!("Sending request to Worker: {}", worker_url);
    println!("Request body: {}", data);

    let response = client
        .post(worker_url)
        .header("Content-Type", "text/plain")
        .body(data.to_owned())
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    println!("Worker response status: {}", status);
    println!("Worker response body: {}", response_text);

    if status.is_success() {
        Ok(response_text)
    } else {
        Err(reqwest::Error::from_body(response))
    }
}