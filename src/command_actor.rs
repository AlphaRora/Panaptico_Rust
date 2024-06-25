use actix::prelude::*;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::error::Error;
use crate::azure_storage_client::AzureDataLakeClient;

macro_rules! create_command_actor {
    ($actor_name:ident, $command:expr, $command_fn:ident, $output_path:expr) => {
        pub struct $actor_name {
            tx: Sender<String>,
            azure_client: Arc<AzureDataLakeClient>,
            output_path: String,
        }

        impl $actor_name {
            pub fn new(tx: Sender<String>, azure_client: Arc<AzureDataLakeClient>) -> Self {
                $actor_name {
                    tx,
                    azure_client,
                    output_path: $output_path.to_string(),
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
                let process = match Command::new("bash")
                    .arg("-c")
                    .arg($command)
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    Ok(process) => process,
                    Err(e) => {
                        eprintln!("Failed to start command: {}", e);
                        ctx.stop();
                        return;
                    }
                };

                let stdout = match process.stdout {
                    Some(stdout) => stdout,
                    None => {
                        eprintln!("Failed to get stdout");
                        ctx.stop();
                        return;
                    }
                };

                let reader = BufReader::new(stdout);
                let reader_mutex = Arc::new(Mutex::new(reader));
                let azure_client = Arc::clone(&self.azure_client);
                let output_path = self.output_path.clone();
                let tx = self.tx.clone();

                ctx.run_interval(std::time::Duration::from_secs(10), move |_act, ctx| {
                    let reader_mutex = Arc::clone(&reader_mutex);
                    let mut reader_guard = match reader_mutex.lock() {
                        Ok(guard) => guard,
                        Err(e) => {
                            eprintln!("Failed to acquire lock: {}", e);
                            ctx.stop();
                            return;
                        }
                    };

                    let mut buffer = String::new();
                    match reader_guard.read_line(&mut buffer) {
                        Ok(0) => {
                            println!("{} command completed", stringify!($actor_name));
                            ctx.stop();
                        },
                        Ok(_) => {
                            let output = buffer.trim_end().to_string();
                            if let Err(e) = tx.send(output.clone()) {
                                eprintln!("Failed to send output: {}", e);
                                ctx.stop();
                                return;
                            }
                            let azure_client = Arc::clone(&azure_client);
                            let output_path = output_path.clone();
                            tokio::spawn(async move {
                                if let Err(e) = azure_client.upload(&output_path, &output).await {
                                    eprintln!("Failed to upload to Azure: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Error reading line: {}", e);
                            ctx.stop();
                        }
                    }
                });
            }
        }
    };
}

// Define command actors with correct commands
create_command_actor!(BashCommandActor, "echo 'Hello, World!'", run_bash_command, "bash_output.txt");
create_command_actor!(SystemInfoCommandActor, "uname -a", run_system_info_command, "system_info_output.txt");
create_command_actor!(NumberOfProcessesCommandActor, "ps -e | wc -l", run_num_procs_command, "num_procs_output.txt");
create_command_actor!(TopProcessCommandActor, "ps -eo pid,comm,%mem,%cpu --sort=-%mem | head -n 2", run_top_proc_command, "top_proc_output.txt");
create_command_actor!(ProcessListCommandActor, "ps -e", run_proc_list_command, "proc_list_output.txt");
create_command_actor!(NetworkLoadCommandActor, "netstat -i | tail -n +3 | awk '{print $3, $4, $5}'", run_load_command, "load_output.txt");
create_command_actor!(DiskUsageCommandActor, "df -h", run_disk_usage_command, "disk_usage_output.txt");

// Main function to start all actors
pub fn start_command_actors(system: &mut actix::System, tx: Sender<String>, azure_client: Arc<AzureDataLakeClient>) {
    let _ = BashCommandActor::new(tx.clone(), Arc::clone(&azure_client)).start();
    let _ = SystemInfoCommandActor::new(tx.clone(), Arc::clone(&azure_client)).start();
    let _ = NumberOfProcessesCommandActor::new(tx.clone(), Arc::clone(&azure_client)).start();
    let _ = TopProcessCommandActor::new(tx.clone(), Arc::clone(&azure_client)).start();
    let _ = ProcessListCommandActor::new(tx.clone(), Arc::clone(&azure_client)).start();
    let _ = NetworkLoadCommandActor::new(tx.clone(), Arc::clone(&azure_client)).start();
    let _ = DiskUsageCommandActor::new(tx.clone(), Arc::clone(&azure_client)).start();
}