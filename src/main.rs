mod command_executor;
mod worker_communication;

use std::sync::mpsc;
use std::thread;
use tokio;

#[tokio::main]
async fn main() {
    let worker_url = "https://newserverworker.adoba.workers.dev/";
    let (tx, rx) = mpsc::channel();
    let (lscpu_tx, lscpu_rx) = mpsc::channel();

    thread::spawn(move || {
        command_executor::execute_bash_command(tx, lscpu_tx);
    });

    tokio::spawn(async move {
        async_loop(rx, &worker_url).await;
    });

    thread::spawn(move || {
        loop {
            let lscpu_output = match lscpu_rx.recv() {
                Ok(output) => output,
                Err(_) => {
                    println!("Error receiving lscpu command output");
                    continue;
                }
            };
            println!("{}", lscpu_output);
            // TODO: Send lscpu_output to the new lscpu bucket
        }
    });
}

async fn async_loop(rx: mpsc::Receiver<String>, worker_url: &str) {
    loop {
        let command_output = match rx.recv() {
            Ok(output) => output,
            Err(_) => {
                println!("Error receiving command output");
                continue;
            }
        };

        println!("Request to Cloudflare Worker was successful. Printing something else.");
        println!("Monitoring wait time for processes targets: tritonserver --model-repository=/mnt/models (PID: 421650 1057344 1339315 1511142 1814944 1818258 1826503 2141245 2324574 2362028 2370337 2716471 3113947 3791815 3820143)");
        println!("---------------------------------------------------------");
        println!("{}", command_output);

        let response = match worker_communication::send_data_request(worker_url, &command_output).await {
            Ok(response) => response,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        if response == "execute_bash_command" {
            println!("Received execute_bash_command response from Worker");
        } else {
            println!("Received unknown response from Worker");
        }
    }
}