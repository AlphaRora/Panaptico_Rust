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

pub fn execute_numberofprocess_command(tx: Sender<String>) -> Result<(), Box<dyn Error>> {
    println!("Executing command to get the total number of processes...");
    let command = "ps -ef | wc -l";
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()?
        .stdout;
    let num_procs = String::from_utf8_lossy(&output).trim().to_string();
    println!("Total number of processes: {}", num_procs);
    tx.send(num_procs)?;
    Ok(())
}

pub fn execute_topprocess_command(tx: Sender<String>) -> Result<(), Box<dyn Error>> {
    println!("Executing command to get the process using the most CPU and memory...");
    let command = "ps aux --no-headers --sort=-pcpu | head -n 1";
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()?
        .stdout;
    let top_proc = String::from_utf8_lossy(&output).trim().to_string();
    println!("Top process: {}", top_proc);
    tx.send(top_proc)?;
    Ok(())
}

pub fn execute_proc_list_command(tx: Sender<String>) -> Result<(), Box<dyn Error>> {
    println!("Executing command to list all processes with CPU and memory utilization...");
    let command = "ps aux --no-headers --sort=-pcpu";
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()?
        .stdout;
    let proc_list = String::from_utf8_lossy(&output).to_string();
    tx.send(proc_list)?;
    Ok(())
}

pub fn execute_network_load_command(tx: Sender<String>) -> Result<(), Box<dyn Error>> {
    println!("Executing command to get network devices and their current load...");
    let command = r#"
#!/bin/bash

# Get the list of network devices
devices=$(netstat -i | awk 'NR>2 {print $1}' | grep -v ^lo)

# Iterate over each device and fetch its current load
for device in $devices
do
    rx_bytes=$(cat /sys/class/net/"$device"/statistics/rx_bytes)
    tx_bytes=$(cat /sys/class/net/"$device"/statistics/tx_bytes)
    rx_packets=$(cat /sys/class/net/"$device"/statistics/rx_packets)
    tx_packets=$(cat /sys/class/net/"$device"/statistics/tx_packets)

    echo "----- $device -----"
    echo "Received Bytes: $rx_bytes"
    echo "Transmitted Bytes: $tx_bytes"
    echo "Received Packets: $rx_packets"
    echo "Transmitted Packets: $tx_packets"
    echo
done
"#;

    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().ok_or("Failed to get child stdout")?;
    let stdout_reader = BufReader::new(stdout);

    for line in stdout_reader.lines() {
        let output = line?;
        println!("Output from network load command: {}", output);
        tx.send(output)?;
    }

    Ok(())
}