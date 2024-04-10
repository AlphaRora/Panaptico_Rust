use std::process::{Command, Stdio};  
use std::io::{BufReader, BufRead};  
use worker_communication;  
  
pub async fn execute_bash_command(request_successful: bool, worker_url: &str) -> Result<(), Box<dyn std::error::Error>> {  
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
        .spawn()  
        .expect("Failed to spawn child process");  
  
    let reader = BufReader::new(child.stdout.take().unwrap());  
  
    for line in reader.lines() {  
        let line = line?;  
        println!("{}", line);  
  
        // Send each line of output to the worker  
        let _ = worker_communication::send_data_request(worker_url, &line).await?;  
    }  
  
    Ok(())  
}  
