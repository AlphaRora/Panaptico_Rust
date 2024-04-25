// main.rs
mod command_executor;
mod worker_communication;
use std::sync::mpsc;
use std::thread;

#[tokio::main]
async fn main() {
    let worker_url = "https://serverworker.adoba.workers.dev/".to_string();
    let glances_url = "https://glances-server.adoba.workers.dev/".to_string();
    let topprocess_url = "https://toprocessworker.adoba.workers.dev/".to_string();
    let numberofprocesses_url = "https://numberofprocessworker.adoba.workers.dev/".to_string();
    let allprocessutilization_url = "https://allprocessutilizationworker.adoba.workers.dev/".to_string();

    // Create channels for each command
    let (bash_tx, bash_rx) = mpsc::channel();
    let (glances_tx, glances_rx) = mpsc::channel();
    let (num_procs_tx, num_procs_rx) = mpsc::channel();
    let (top_proc_tx, top_proc_rx) = mpsc::channel();
    let (proc_list_tx, proc_list_rx) = mpsc::channel();

    // Spawn a separate thread to execute the glances command
    let glances_handle = thread::spawn(move || {
        if let Err(err) = command_executor::execute_glances_command(glances_tx) {
            eprintln!("Error executing glances command: {:?}", err);
        } else {
            eprintln!("good job");
        }
    });

    // Spawn another thread to execute the tritonserver command
    let bash_handle = thread::spawn(move || {
        if let Err(err) = command_executor::execute_bash_command(bash_tx) {
            eprintln!("Error executing bash command: {:?}", err);
        } else {
            eprintln!("bash job");
        }
    });

    let num_procs_handle = thread::spawn(move || {
        if let Err(err) = command_executor::execute_numberofprocess_command(num_procs_tx) {
            eprintln!("Error executing num_procs command: {:?}", err);
        } else {
            eprintln!("num_procs job");
        }
    });

    let top_proc_handle = thread::spawn(move || {
        if let Err(err) = command_executor::execute_topprocess_command(top_proc_tx) {
            eprintln!("Error executing top_proc command: {:?}", err);
        } else {
            eprintln!("top_proc job");
        }
    });

    let proc_list_handle = thread::spawn(move || {
        if let Err(err) = command_executor::execute_proc_list_command(proc_list_tx) {
            eprintln!("Error executing proc_list command: {:?}", err);
        } else {
            eprintln!("proc_list job");
        }
    });

    // Handle output from all commands concurrently
    let glances_worker = handle_glances_output(glances_rx, glances_url);
    let bash_worker = handle_bash_output(bash_rx, worker_url);
    let num_procs_worker = handle_num_procs_output(num_procs_rx, numberofprocesses_url );
    let top_proc_worker = handle_top_proc_output(top_proc_rx, topprocess_url);
    let proc_list_worker = handle_proc_list_output(proc_list_rx, allprocessutilization_url);

    // Wait for all workers to complete
    tokio::join!(
        glances_worker,
        bash_worker,
        num_procs_worker,
        top_proc_worker,
        proc_list_worker
    );
}

async fn handle_glances_output(glances_rx: mpsc::Receiver<String>, glances_url: String) {
    for command_output in glances_rx {
        println!("Received output from glances command");
        if !command_output.trim().is_empty() {
            println!("Output from sudo glances command:");
            println!("{}", command_output);
            // Send data to the Glances Worker
            let response = match worker_communication::send_glances_data_request(&glances_url, &command_output).await {
                Ok(response) => response,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };
            // Check the response from the Glances Worker
            if response == "execute_glances_command" {
                println!("Received execute_glances_command response from Glances Worker");
            } else {
                println!("Received unknown response from Glances Worker: {}", response);
            }
        }
    }
}

async fn handle_bash_output(bash_rx: mpsc::Receiver<String>, worker_url: String) {
    for command_output in bash_rx {
        println!("Request to Cloudflare Worker was successful. Printing something else.");
        println!("{}", command_output);
        let response = match worker_communication::send_data_request(&worker_url, &command_output).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        // Check the response from the Worker
        if response == "execute_bash_command" {
            println!("Received execute_bash_command response from Worker");
        } else {
            println!("Received unknown response from Worker, issue is with bash: {}", response);
        }
    }
}

async fn handle_num_procs_output(num_procs_rx: mpsc::Receiver<String>, numberofprocesses_url: String) {
    for command_output in num_procs_rx {
        println!("Total number of processes: {}", command_output);
        let response = match worker_communication::send_processes_count_request(&numberofprocesses_url, &command_output).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        // Check the response from the Worker
        if response == "execute_bash_command" {
            println!("Received execute_bash_command response from Worker");
        } else {
            println!("Received unknown response from Worker, issue is with bash: {}", response);
        }
    }
}

async fn handle_top_proc_output(top_proc_rx: mpsc::Receiver<String>, topprocess_url: String) {
    for command_output in top_proc_rx {
        println!("Top process: {}", command_output);
        let response = match worker_communication::send_top_process_request(&topprocess_url, &command_output).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        // Check the response from the Worker
        if response == "execute_bash_command" {
            println!("Received execute_bash_command response from Worker");
        } else {
            println!("Received unknown response from Worker, issue is with bash: {}", response);
        }
    }
}

async fn handle_proc_list_output(proc_list_rx: mpsc::Receiver<String>, allprocessutilization_url: String) {
    for command_output in proc_list_rx {
        println!("Process list:\n{}", command_output);
        let response = match worker_communication::send_process_utlization_request(&allprocessutilization_url, &command_output).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        // Check the response from the Worker
        if response == "execute_bash_command" {
            println!("Received execute_bash_command response from Worker");
        } else {
            println!("Received unknown response from Worker, issue is with bash: {}", response);
        }
    }
    }

