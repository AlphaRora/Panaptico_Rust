// command_executor.rs
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::io::{BufRead, BufReader};

pub fn execute_bash_command(tx: Sender<String>, lscpu_tx: Sender<String>) {    
    // let command = r#"     
    // interval=5;     
    // process_name="tritonserver --model-repository=/mnt/models";     
    // pid=$(pgrep -f "$process_name");     
    // if [[ -z "$pid" ]]; then     
    //     echo "Error: Inference process not found. Please provide the correct process name.";     
    //     exit 1;     
    // fi;     
    // echo "Monitoring wait time for processes targets: $process_name (PID: $pid)";     
    // echo "---------------------------------------------------------";     
    // while true; do     
    //     iostat -d -x 1 $interval | tail -n +3;     
    //     pidstat -d -p $pid $interval | tail -n +4 | awk '{print "I/O Wait (%): " $11}';       
    //     echo "---------------------------------------------------------";     
    // done "#;    
  
    // let mut child = Command::new("bash")    
    //     .arg("-c")    
    //     .arg(command)    
    //     .stdout(Stdio::piped())    
    //     .spawn()    
    //     .expect("Failed to spawn child process");    
    
    // let stdout = child.stdout.take().expect("Failed to get child stdout");    
    
    // let stdout_reader = BufReader::new(stdout);    
    
    // for line in stdout_reader.lines() {    
    //     let output = line.expect("Failed to read line from child stdout");    
    //     tx.send(output).expect("Failed to send command output");    
    // }  
  
    // New lscpu command execution    
    let mut lscpu_child = Command::new("lscpu")    
        .stdout(Stdio::piped())    
        .spawn()    
        .expect("Failed to spawn lscpu child process");    
    
    let lscpu_stdout = lscpu_child.stdout.take().expect("Failed to get lscpu child stdout");    
    let lscpu_stdout_reader = BufReader::new(lscpu_stdout);    
    
    for line in lscpu_stdout_reader.lines() {    
        let lscpu_output = line.expect("Failed to read line from lscpu child stdout");    
        lscpu_tx.send(lscpu_output).expect("Failed to send lscpu command output");    
    }    
}  
