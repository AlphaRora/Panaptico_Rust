// command_executor.rs
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::io::{BufRead, BufReader};

pub fn execute_bash_command(tx: Sender<String>, lscpu_tx: Sender<String>) {  
    let mut lscpu_child = Command::new("lscpu")  
        .stdout(Stdio::piped())  
        .spawn()  
        .expect("Failed to spawn lscpu child process");  
  
    let lscpu_stdout = lscpu_child.stdout.take().expect("Failed to get lscpu child stdout");  
    let lscpu_stdout_reader = BufReader::new(lscpu_stdout);  
  
    for line in lscpu_stdout_reader.lines() {  
        match line {  
            Ok(lscpu_output) => {  
                if let Err(e) = lscpu_tx.send(lscpu_output) {  
                    eprintln!("Failed to send lscpu command output: {}", e);  
                }  
            }  
            Err(e) => eprintln!("Failed to read line from lscpu child stdout: {}", e),  
        }  
    }  
}  
