// main.rs
mod command_executor;
mod worker_communication;

use std::sync::mpsc;
use std::thread;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";
    let (tx, rx) = mpsc::channel();

    // Spawn a separate thread to execute the tritonserver command
    thread::spawn(move || {
        command_executor::execute_bash_command(tx.clone());
    });

    // Spawn another thread to execute the glances command
    thread::spawn(move || {
        command_executor::execute_glances_command(tx.clone());
    });

    loop {
        // Receive the command output from the channel
        let command_output = match rx.recv() {
            Ok(output) => output,
            Err(_) => {
                println!("Error receiving command output");
                continue;
            }
        };

        // Determine the source of the command output and handle it accordingly
        if command_output.contains("Monitoring wait time for processes targets") {
            // Output from the tritonserver command
            println!("Request to Cloudflare Worker was successful. Printing something else.");
            println!("{}", command_output);
        } else {
            // Output from the glances command
            println!("Output from sudo glances command:");
            println!("{}", command_output);

            // Send data to the Worker
            let response = match worker_communication::send_data_request(&worker_url, &command_output).await {
                Ok(response) => response,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };

            // Check the response from the Worker
            if response == "execute_glances_command" {
                println!("Received execute_glances_command response from Worker");
            } else {
                println!("Received unknown response from Worker");
            }
        }
    }
}
