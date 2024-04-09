// src/command_executor.rs
use std::process::Command;

pub fn execute_bash_command() {
    let command = r#"
        interval=5;
        process_name="tritonserver --model-repository=/mnt/models";
        pid=$(pgrep -f "$process_name");
        if [[ -z "$pid" ]]; then
            echo "Error: Inference process not found. Please provide the correct process name.";
            exit 1;
        fi;
        echo "Monitoring disk I/O wait time for process: $process_name (PID: $pid)";
        echo "---------------------------------------------------------";
        while true; do
            iostat -d -x 1 $interval | tail -n +3;
            pidstat -d -p $pid $interval | tail -n +4 | awk '{print "I/O Wait (%): " $11}';
            echo "---------------------------------------------------------";
        done
    "#;

    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to spawn child process");

    // Handle the child process output if needed
    // ...
}