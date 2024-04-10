use std::process::Command;

mod command_executor;
mod worker_communication;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/";

    loop {
        // Execute the command and capture its output
        let output = Command::new("bash")
            .arg("-c")
            .arg(r#"
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
            "#)
            .output()
            .expect("Failed to execute command");

        let command_output = String::from_utf8_lossy(&output.stdout).into_owned();

        // Send the command output to the Worker
        let response = match worker_communication::send_data_request(&worker_url, &command_output).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        // Check the response from the Worker
        if response == "execute_bash_command" {
            command_executor::execute_bash_command(true);
        } else {
            command_executor::execute_bash_command(false);
        }
        // Add more conditions or logic to handle different commands
    }
}