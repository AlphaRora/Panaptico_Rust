// src/command_executor.rs
use std::process::Command;

pub fn execute_bash_command(request_successful: bool) -> Result<String, std::io::Error> {
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
        iostat -d -x 1 $interval | tail -n +3;
        pidstat -d -p $pid $interval | tail -n +4 | awk '{print "I/O Wait (%): " $11}';
        echo "---------------------------------------------------------";
    "#;

    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}