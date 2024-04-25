// command_executor.rs
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;



pub fn execute_bash_command(tx: Sender<String>) -> Result<(), Box<dyn Error>> {
    println!("Executing bash command...");

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
        echo "Iteration start"
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

    println!("Bash command spawned successfully.");

    let stdout = child.stdout.take().ok_or("Failed to get child stdout")?;
    let stdout_reader = BufReader::new(stdout);

    for line in stdout_reader.lines() {
        let output = line?;
        println!("Output from bash command: {}", output);
        tx.send(output)?;
    }

    Ok(())
}

pub fn execute_glances_command(tx: Sender<String>) -> Result<(), Box<dyn Error>> {
    println!("Executing glances command...");
    let command = r#"sudo glances --export csv --export-csv-file=/tmp/glances.csv"#;

    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .spawn()?;

    child.wait()?;

    let file = File::open("/tmp/glances.csv")?;
    let reader = BufReader::new(file);
    println!("Glances might be working!");

    for line in reader.lines() {
        let output = line?;
        println!("Output from glances command: {}", output);
        tx.send(output)?;
    }

    Ok(())
}

