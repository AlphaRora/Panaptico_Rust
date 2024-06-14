mod websocket_actor;
mod supervisor;
mod command_actors;
mod azure_storage_client;

use actix::Actor;
use actix_rt::System;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tokio_tungstenite::accept_async;
use websocket_actor::WebSocketActor;
use supervisor::SupervisorActor;
use command_actors::*;
use azure_storage_client::AzureDataLakeClient;
use std::sync::Arc;

#[actix::main]
async fn main() {
    let sys = System::new();
    SupervisorActor::start_supervisor();

    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");
    println!("WebSocket server listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream)
            .await
            .expect("Failed to accept WebSocket connection");
        WebSocketActor::new(ws_stream).start();
    }

    let azure_client = Arc::new(AzureDataLakeClient::new(
        "datalakestoragepanaptico",
        "machinelogs", // replace with your actual container name
    ));

    let (bash_tx, bash_rx) = std::sync::mpsc::channel();
    let (glances_tx, glances_rx) = std::sync::mpsc::channel();
    let (num_procs_tx, num_procs_rx) = std::sync::mpsc::channel();
    let (top_proc_tx, top_proc_rx) = std::sync::mpsc::channel();
    let (proc_list_tx, proc_list_rx) = std::sync::mpsc::channel();
    let (load_list_tx, load_list_rx) = std::sync::mpsc::channel();
    let (speed_list_tx, speed_list_rx) = std::sync::mpsc::channel();

    BashCommandActor::new(bash_tx, Arc::clone(&azure_client)).start();
    GlancesCommandActor::new(glances_tx, Arc::clone(&azure_client)).start();
    NumberOfProcessesCommandActor::new(num_procs_tx, Arc::clone(&azure_client)).start();
    TopProcessCommandActor::new(top_proc_tx, Arc::clone(&azure_client)).start();
    ProcessListCommandActor::new(proc_list_tx, Arc::clone(&azure_client)).start();
    NetworkLoadCommandActor::new(load_list_tx, Arc::clone(&azure_client)).start();
    NetworkSpeedCommandActor::new(speed_list_tx, Arc::clone(&azure_client)).start();

    // Spawn tasks to handle the outputs from each command actor
    tokio::spawn(handle_bash_output(bash_rx, Arc::clone(&azure_client)));
    tokio::spawn(handle_glances_output(glances_rx, Arc::clone(&azure_client)));
    tokio::spawn(handle_num_procs_output(num_procs_rx, Arc::clone(&azure_client)));
    tokio::spawn(handle_top_proc_output(top_proc_rx, Arc::clone(&azure_client)));
    tokio::spawn(handle_proc_list_output(proc_list_rx, Arc::clone(&azure_client)));
    tokio::spawn(handle_load_output(load_list_rx, Arc::clone(&azure_client)));
    tokio::spawn(handle_speed_output(speed_list_rx, Arc::clone(&azure_client)));

    sys.run().await.unwrap();
}

async fn handle_bash_output(bash_rx: std::sync::mpsc::Receiver<String>, azure_client: Arc<AzureDataLakeClient>) {
    for command_output in bash_rx {
        println!("Output from bash command: {}", command_output);
        azure_client.upload("bash_output.txt", &command_output).await.unwrap();
    }
}

async fn handle_glances_output(glances_rx: std::sync::mpsc::Receiver<String>, azure_client: Arc<AzureDataLakeClient>) {
    for command_output in glances_rx {
        println!("Output from glances command: {}", command_output);
        azure_client.upload("glances_output.txt", &command_output).await.unwrap();
    }
}

async fn handle_num_procs_output(num_procs_rx: std::sync::mpsc::Receiver<String>, azure_client: Arc<AzureDataLakeClient>) {
    for command_output in num_procs_rx {
        println!("Total number of processes: {}", command_output);
        azure_client.upload("num_procs_output.txt", &command_output).await.unwrap();
    }
}

async fn handle_top_proc_output(top_proc_rx: std::sync::mpsc::Receiver<String>, azure_client: Arc<AzureDataLakeClient>) {
    for command_output in top_proc_rx {
        println!("Top process: {}", command_output);
        azure_client.upload("top_proc_output.txt", &command_output).await.unwrap();
    }
}

async fn handle_proc_list_output(proc_list_rx: std::sync::mpsc::Receiver<String>, azure_client: Arc<AzureDataLakeClient>) {
    for command_output in proc_list_rx {
        println!("Process list:\n{}", command_output);
        azure_client.upload("proc_list_output.txt", &command_output).await.unwrap();
    }
}

async fn handle_load_output(load_rx: std::sync::mpsc::Receiver<String>, azure_client: Arc<AzureDataLakeClient>) {
    for command_output in load_rx {
        println!("Network load: {}", command_output);
        azure_client.upload("load_output.txt", &command_output).await.unwrap();
    }
}

async fn handle_speed_output(speed_rx: std::sync::mpsc::Receiver<String>, azure_client: Arc<AzureDataLakeClient>) {
    for command_output in speed_rx {
        println!("Network speed: {}", command_output);
        azure_client.upload("speed_output.txt", &command_output).await.unwrap();
    }
}
