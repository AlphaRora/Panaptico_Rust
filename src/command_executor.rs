use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

pub fn execute_bash_command(request_successful: bool) -> String {
    if !request_successful {
        println!("Request to Cloudflare Worker failed. Skipping command execution.");
        return String::new();
    }

    println!("Request to Cloudflare Worker was successful. Printing something else.");

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

    let stdout = child.stdout.as_mut().unwrap();
    let reader = BufReader::new(stdout);

    let mut output = String::new();
    reader.lines().for_each(|line| {
        if let Ok(line) = line {
            output.push_str(&line);
            output.push('\n');
        }
    });

    output
}