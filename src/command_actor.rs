use actix::prelude::*;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::azure_storage_client::AzureDataLakeClient;

macro_rules! create_command_actor {
    ($actor_name:ident, $command:expr, $command_fn:ident, $output_path:expr) => {
        pub struct $actor_name {
            tx: Sender<String>,
            azure_client: Arc<AzureDataLakeClient>,
            output_path: String,
            reader_mutex: Arc<Mutex<BufReader<std::process::ChildStdout>>>,
        }

        impl $actor_name {
            pub fn new(tx: Sender<String>, azure_client: Arc<AzureDataLakeClient>) -> Self {
                let process = Command::new("bash")
                    .arg("-c")
                    .arg($command)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Failed to start command");

                let stdout = process.stdout.expect("Failed to get stdout");
                let reader = BufReader::new(stdout);
                let reader_mutex = Arc::new(Mutex::new(reader));

                $actor_name {
                    tx,
                    azure_client,
                    output_path: $output_path.to_string(),
                    reader_mutex,
                }
            }
        }

        impl Actor for $actor_name {
            type Context = Context<Self>;

            fn started(&mut self, ctx: &mut Self::Context) {
                println!("{} started", stringify!($actor_name));
                self.$command_fn(ctx);
            }
        }

        impl $actor_name {
            fn $command_fn(&self, ctx: &mut <Self as Actor>::Context) {
                let reader_mutex = Arc::clone(&self.reader_mutex);
                let azure_client = Arc::clone(&self.azure_client);
                let output_path = self.output_path.clone();
                let tx = self.tx.clone();

                ctx.run_interval(std::time::Duration::from_secs(10), move |act, _| {
                    let mut reader = reader_mutex.lock().unwrap();
                    for line in reader.lines() {
                        match line {
                            Ok(output) => {
                                tx.send(output.clone()).expect("Failed to send output");
                                let azure_client = Arc::clone(&azure_client);
                                let output_path = output_path.clone();
                                tokio::spawn(async move {
                                    azure_client.upload(&output_path, &output).await.unwrap();
                                });
                            }
                            Err(e) => println!("Error reading line: {}", e),
                        }
                    }
                });
            }
        }
    };
}

// Define AzureDataLakeClient if it's not imported from another module
pub struct AzureDataLakeClient;

impl AzureDataLakeClient {
    pub async fn upload(&self, output_path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Example implementation of upload function
        println!("Uploading data to Azure Data Lake at path: {}", output_path);
        println!("Data: {}", data);
        // Simulate upload delay
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("Upload complete");
        Ok(())
    }
}

// Example usage of the macro to create different command actors
create_command_actor!(BashCommandActor, "some bash command", run_bash_command, "bash_output.txt");
// Add similar create_command_actor! macros for other commands as needed

// Example main function for testing
#[actix_rt::main]
async fn main() {
    // Example usage of the actors
    let azure_client = Arc::new(AzureDataLakeClient {});
    let (tx, rx) = std::sync::mpsc::channel::<String>();

    let bash_actor = BashCommandActor::new(tx.clone(), Arc::clone(&azure_client));
    let glances_actor = GlancesCommandActor::new(tx.clone(), Arc::clone(&azure_client));
    let num_procs_actor = NumberOfProcessesCommandActor::new(tx.clone(), Arc::clone(&azure_client));
    let top_proc_actor = TopProcessCommandActor::new(tx.clone(), Arc::clone(&azure_client));
    let proc_list_actor = ProcessListCommandActor::new(tx.clone(), Arc::clone(&azure_client));
    let load_actor = NetworkLoadCommandActor::new(tx.clone(), Arc::clone(&azure_client));
    let speed_actor = NetworkSpeedCommandActor::new(tx.clone(), Arc::clone(&azure_client));

    let _ = bash_actor.start();
    let _ = glances_actor.start();
    let _ = num_procs_actor.start();
    let _ = top_proc_actor.start();
    let _ = proc_list_actor.start();
    let _ = load_actor.start();
    let _ = speed_actor.start();

    // Example: Listen to outputs from actors
    tokio::spawn(async move {
        while let Ok(output) = rx.recv() {
            println!("Received output: {}", output);
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
}
