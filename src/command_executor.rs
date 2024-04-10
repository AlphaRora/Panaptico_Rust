// src/command_executor.rs
// use std::process::Command;
use std::process::{Command, Stdio, Output};

pub fn execute_bash_command(request_successful: bool) -> Result<String, std::io::Error> {

    if !request_successful {
        println!("Request to Cloudflare Worker failed. Skipping command execution.");
        return;
    } else {
        println!("Request to Cloudflare Worker was successful. Printing something else.");
    }
    

    let command = r#"
        interval=5;
        process_name="tritonserver --model-repository=/mnt/models";
        pid=$(pgrep -f "$process_name");
        if [[ -z "$pid" ]]; then
            echo "Error: Inference process not found. Please provide the correct process name.";
            exit 1;
        fi;
        echo "Monitoring wait time for processes targets: $process_name (PID: $pid)";
        echo "---------------------------------------------------------";
        while true; do
            iostat -d -x 1 $interval | tail -n +3;
            pidstat -d -p $pid $interval | tail -n +4 | awk '{print "I/O Wait (%): " $11}';
            echo "---------------------------------------------------------";
        done
    "#;

    let output: Output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .output()?; // Change here to directly get output

    let command_output = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(command_output) // Return the command output
    // Handle the child process output if needed
    // ...
}
