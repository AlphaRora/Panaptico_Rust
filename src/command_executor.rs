// command_executor.rs
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::io::{BufRead, BufReader};

pub fn execute_bash_command(tx: Sender<String>, lscpu_tx: Sender<String>) {  
    // New lscpu command execution  
    let output = Command::new("lscpu")  
        .output()  
        .expect("Failed to execute lscpu command");  
  
    // Check for errors from the lscpu command  
    if output.status.success() {  
        let lscpu_output_str = String::from_utf8_lossy(&output.stdout);  
        lscpu_tx.send(lscpu_output_str.to_string()).expect("Failed to send lscpu command output");  
    } else {  
        eprintln!("lscpu command failed: {}", String::from_utf8_lossy(&output.stderr));  
    }  
}  
