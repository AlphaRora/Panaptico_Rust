// src/command_executor.rs
use std::process::Command;
use std::process::{Command, Stdio};

pub fn execute_bash_command(request_successful: bool) -> Result<String, String> {
    if !request_successful {
        println!("Request to Cloudflare Worker failed. Skipping command execution.");
        return Ok(String::new());
    } else {
        println!("Request to Cloudflare Worker was successful. Executing command.");
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

    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()
        .and_then(|child| child.wait_with_output())
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    let output_string = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to convert output to string: {}", e))?;

    Ok(output_string)
}