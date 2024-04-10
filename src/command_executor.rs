// src/command_executor.rs
use std::process::Command;



pub fn execute_bash_command(request_successful: bool) -> Result<String, std::io::Error> {  
    if !request_successful {  
        println!("Request to Cloudflare Worker failed. Skipping command execution.");  
        return Ok(String::from("Request failed")); // You can change this to return an Err if you prefer  
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

    // let mut child = Command::new("bash")
    // .arg("-c")
    // .arg(command)
    // .spawn()
    // .expect("Failed to spawn child process");
  
    let output = Command::new("bash").arg("-c").arg(command).output()?;    
    let output_str = String::from_utf8_lossy(&output.stdout).into_owned();    
    Ok(output_str)  
}  

