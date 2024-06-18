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
                let process = Command::new("bash")
                    .arg("-c")
                    .arg($command)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Failed to start command");

                let stdout = process.stdout.expect("Failed to get stdout");
                let reader = BufReader::new(stdout);
                let reader_mutex = Arc::new(Mutex::new(reader));

                let azure_client = Arc::clone(&self.azure_client);
                let output_path = self.output_path.clone();
                let tx = self.tx.clone();

                ctx.run_interval(std::time::Duration::from_secs(10), move |act, _| {
                    let mut reader = reader_mutex.lock().unwrap();
                    for line in reader.lines() {
                        match line {
                            Ok(output) => {
                                let output = output.clone();
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

create_command_actor!(BashCommandActor, "some bash command", run_bash_command, "bash_output.txt");
create_command_actor!(GlancesCommandActor, "glances --stdout", run_glances_command, "glances_output.txt");
create_command_actor!(NumberOfProcessesCommandActor, "ps -e | wc -l", run_num_procs_command, "num_procs_output.txt");
create_command_actor!(TopProcessCommandActor, "ps -eo pid,comm,%mem,%cpu --sort=-%mem | head -n 2", run_top_proc_command, "top_proc_output.txt");
create_command_actor!(ProcessListCommandActor, "ps -e", run_proc_list_command, "proc_list_output.txt");
create_command_actor!(NetworkLoadCommandActor, "some network load command", run_load_command, "load_output.txt");
create_command_actor!(NetworkSpeedCommandActor, "some network speed command", run_speed_command, "speed_output.txt");
