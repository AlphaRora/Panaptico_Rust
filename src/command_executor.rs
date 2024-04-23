use std::process::{Command, Stdio};
use std::sync::mpsc::{Sender, channel};
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;

pub fn execute_bash_command(tx: Sender<String>) {
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
        .spawn()
        .expect("Failed to spawn child process");

    let stdout = child.stdout.take().expect("Failed to get child stdout");
    let stdout_reader = BufReader::new(stdout);

    for line in stdout_reader.lines() {
        let output = line.expect("Failed to read line from child stdout");
        tx.send(output).expect("Failed to send command output");
    }
}

pub fn get_network_health(tx: Sender<String>) {
    thread::spawn(move || {
        let ping_result = Command::new("ping")
            .args(&["-c", "1", "(link unavailable)"])
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute ping command");

        let ping_output = String::from_utf8_lossy(&ping_result.stdout);
        let connectivity = ping_result.status.success();
        let latency = if connectivity {
            let latency_match = regex::Regex::new(r"time=([0-9.]+) ms")
                .unwrap()
                .captures(&ping_output)
                .and_then(|caps| caps.get(1))
                .map(|m| m.as_str().parse::<f64>().unwrap());
            latency_match
        } else {
            None
        };

        // Implement other network health checks here

        // Example sending data through the channel
        tx.send(format!("Connectivity: {}", connectivity)).expect("Failed to send connectivity data");

        // Simulate sending other network health data through the channel
        thread::sleep(Duration::from_secs(1));
        tx.send("Other network health data".to_string()).expect("Failed to send other network health data");
    });
}

pub fn main() {
    let (tx, rx) = channel();

    // Spawn threads for network health check and command execution
    get_network_health(tx.clone());
    execute_bash_command(tx.clone());

    // Example receiving data from the channel
    for received in rx {
        println!("{}", received);
    }
}
