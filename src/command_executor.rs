use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Error, ErrorKind};

pub async fn execute_bash_command(request_successful: bool) -> Result<(), std::io::Error> {
    if !request_successful {
        println!("Request to Cloudflare Worker failed. Skipping command execution.");
        return Ok(());
    }

    let command = r#"
        interval=5;
        process_name="tritonserver --model-repository=/mnt/models";
        pid=$(pgrep -f "$process_name");
        if [ -z "$pid" ]; then
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

    let stdout = child.stdout.as_mut().expect("Failed to get stdout");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = line?;
        let worker_url = "https://serverworker.adoba.workers.dev/";
        match worker_communication::send_data_request(&worker_url, &line).await {
            Ok(_) => (),
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        }
    }

    Ok(())
}