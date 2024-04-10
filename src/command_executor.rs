// src/command_executor.rs
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;

pub fn execute_bash_command(
    request_successful: bool,
    send_data: &mut dyn FnMut(String) -> Result<(), reqwest::Error>,
) -> Result<(), std::io::Error> {
    if !request_successful {
        println!("Request to Cloudflare Worker failed. Skipping command execution.");
        return Ok(());
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

    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().unwrap();
    let stdout_reader = BufReader::new(stdout);

    tokio::task::spawn_blocking(move || {
        for line in stdout_reader.lines() {
            if let Ok(output) = line {
                if let Err(e) = send_data(output) {
                    eprintln!("Error sending data to Worker: {}", e);
                }
            }
            std::thread::sleep(Duration::from_secs(5)); // Adjust the delay as needed
        }
    });

    Ok(())
}