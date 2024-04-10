use std::process::{Command, Output, ExitStatus};  
  
pub fn execute_bash_command(request_successful: bool) -> Option<Output> {  
    if !request_successful {  
        println!("Request to Cloudflare Worker failed. Skipping command execution.");  
        return None;  
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
  
    let output = Command::new("bash")  
        .arg("-c")  
        .arg(command)  
        .output()  
        .ok();  
  
    output  
}  
