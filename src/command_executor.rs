// command_executor.rs
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::io::{BufRead, BufReader};

pub fn execute_bash_command(tx: Sender<String>, lscpu_tx: Sender<String>) {  
    // New lscpu command execution  
    let lscpu_output = Command::new("lscpu")  
        .output()  
        .expect("Failed to execute lscpu command");  
  
    // The output is a Result type, we need to handle it  
    match lscpu_output {  
        Ok(output) => {  
            let lscpu_output_str = String::from_utf8_lossy(&output.stdout);  
            lscpu_tx.send(lscpu_output_str.to_string()).expect("Failed to send lscpu command output");  
        },  
        Err(e) => {  
            eprintln!("lscpu command failed: {}", e);  
        }  
    }  
}  
