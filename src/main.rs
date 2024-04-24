// main.rs
mod command_executor;
mod worker_communication;
use std::sync::mpsc::Sender;
use std::thread;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::error::Error;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";
    let glances_url = "https://glancesworker.adoba.workers.dev/";

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

    // Wait for the threads to complete
    // bash_handle.join().unwrap();
    // glances_handle.join().unwrap();

    // Receive and handle output from the tritonserver command
    for command_output in bash_rx {
        println!("Request to Cloudflare Worker was successful. Printing something else.");
        println!("{}", command_output);
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
            println!("Received unknown response from Worker, issue is with glances: {}", response);
        }
    }
    

    // Receive and handle output from the glances command
    // for command_output in glances_rx {
    //     println!("Received output from glances command: {}", command_output);
    //     if !command_output.trim().is_empty() {
    //         println!("Output from sudo glances command:");
    //         println!("{}", command_output);
    //         // Send data to the Worker
    //         let response = match worker_communication::send_data_request(&worker_url, &command_output).await {
    //             Ok(response) => response,
    //             Err(e) => {
    //                 println!("Error: {}", e);
    //                 continue;
    //             }
    //         };
    //         // Check the response from the Worker
    //         if response == "execute_glances_command" {
    //             println!("Received execute_glances_command response from Worker");
    //         } else {
    //             println!("Received unknown response from Worker");
    //         }
    //     }
    // }

    pub fn execute_glances_command(tx: Sender<String>) -> Result<(), Box<dyn Error>> {
        println!("Executing glances command...");
        let command = r#"sudo glances --export csv --export-csv-file=/tmp/glances.csv"#;
        let mut child = Command::new("bash")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .spawn()?;
    
        let stdout = child.wait_with_output()?;
    
        if !stdout.status.success() {
            return Err(format!("Failed to execute glances command: {}", String::from_utf8_lossy(&stdout.stderr)).into());
        }
    
        let csv_file = std::fs::read_to_string("/tmp/glances.csv")?;
        tx.send(csv_file)?;
    
        Ok(())
    }
}
