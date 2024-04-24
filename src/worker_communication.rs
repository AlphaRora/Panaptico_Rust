use reqwest::Client;  
  
pub async fn send_data_request(worker_url: &str, data: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post(worker_url)
        .header("Content-Type", "text/plain")
        .body(data.to_owned())
        .send()
        .await?
        .text()
        .await?;

    println!("Worker response: {}", response);
    Ok(response)
}

pub async fn send_glances_data_request(glances_url: &str, data: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post(glances_url)
        .header("Content-Type", "text/plain")
        .body(data.to_owned())
        .send()
        .await?
        .text()
        .await?;

    println!("Glances Worker response: {}", response);
    Ok(response)
}
