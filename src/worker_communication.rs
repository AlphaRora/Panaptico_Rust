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

pub async fn send_processes_count_request(numberofprocesses_url: &str, data: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post(numberofprocesses_url)
        .header("Content-Type", "text/plain")
        .body(data.to_owned())
        .send()
        .await?
        .text()
        .await?;

    println!("Process Count response: {}", response);
    Ok(response)
}

pub async fn send_top_process_request(topprocess_url: &str, data: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post(topprocess_url)
        .header("Content-Type", "text/plain")
        .body(data.to_owned())
        .send()
        .await?
        .text()
        .await?;

    println!("Top Process response: {}", response);
    Ok(response)
}

pub async fn send_process_utlization_request(allprocessutilization_url: &str, data: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post(allprocessutilization_url)
        .header("Content-Type", "text/plain")
        .body(data.to_owned())
        .send()
        .await?
        .text()
        .await?;

    println!("Process Utilization: {}", response);
    Ok(response)
}
