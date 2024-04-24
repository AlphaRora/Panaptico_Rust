// main.rs
mod command_executor;
mod worker_communication;

use std::sync::mpsc;
use std::thread;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";

    // Create channels for each command
    let (bash_tx, bash_rx) = mpsc::channel();
    let (glances_tx, glances_rx) = mpsc::channel();

    // Spawn a separate thread to execute the tritonserver command
    let bash_handle = thread::spawn(move || {
        if let Err(err) = command_executor::execute_bash_command(bash_tx) {
            eprintln!("Error executing bash command: {:?}", err);
        }
    });

    // Spawn another thread to execute the glances command
    let glances_handle = thread::spawn(move || {
        if let Err(err) = command_executor::execute_glances_command(glances_tx) {
            eprintln!("Error executing glances command: {:?}", err);
        }
    });

    // Join both threads
    if let Err(err) = bash_handle.join() {
        eprintln!("Error joining bash thread: {:?}", err);
    }
    if let Err(err) = glances_handle.join() {
        eprintln!("Error joining glances thread: {:?}", err);
    }

    // Receive and handle output from the tritonserver command
    for command_output in bash_rx {
        println!("Request to Cloudflare Worker was successful. Printing something else.");
        println!("{}", command_output);
    }

    // Receive and handle output from the glances command
    for command_output in glances_rx {
        if !command_output.trim().is_empty() {
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
