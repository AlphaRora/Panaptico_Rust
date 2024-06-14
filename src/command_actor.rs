// command_actors.rs
use actix::prelude::*;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
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
                $actor_name { tx, azure_client, output_path: $output_path.to_string() }
            }
        }

        impl Actor for $actor_name {
            type Context = Context<Self>;
        }

        pub struct $command_fn;

        impl Message for $command_fn {
            type Result = Result<(), Box<dyn Error>>;
        }

        impl Handler<$command_fn> for $actor_name {
            type Result = Result<(), Box<dyn Error>>;

            fn handle(&mut self, _: $command_fn, _: &mut Self::Context) -> Self::Result {
                let mut child = Command::new("bash")
                    .arg("-c")
                    .arg($command)
                    .stdout(Stdio::piped())
                    .spawn()?;

                let stdout = child.stdout.take().ok_or("Failed to get child stdout")?;
                let stdout_reader = BufReader::new(stdout);
                let mut output = String::new();

                for line in stdout_reader.lines() {
                    let output_line = line?;
                    self.tx.send(output_line.clone())?;
                    output.push_str(&output_line);
                    output.push('\n');
                }

                let azure_client = Arc::clone(&self.azure_client);
                let output_path = self.output_path.clone();
                Arbiter::spawn(async move {
                    let _ = azure_client.upload_data(&output_path, &output).await;
                });

                Ok(())
            }
        }
    };
}

// Define actors for each command using the macro, specifying the output path
create_command_actor!(
    BashCommandActor,
    r#"
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
    "#,
    ExecuteBashCommand,
    "bash_command_output.txt"
);

create_command_actor!(
    GlancesCommandActor,
    "sudo glances --export csv --export-csv-file=/tmp/glances.csv",
    ExecuteGlancesCommand,
    "glances_output.txt"
);

create_command_actor!(
    NumberOfProcessesCommandActor,
    "ps -ef | wc -l",
    ExecuteNumberOfProcessesCommand,
    "num_procs_output.txt"
);

create_command_actor!(
    TopProcessCommandActor,
    "ps aux --no-headers --sort=-pcpu | head -n 1",
    ExecuteTopProcessCommand,
    "top_proc_output.txt"
);

create_command_actor!(
    ProcessListCommandActor,
    "ps aux --no-headers --sort=-pcpu",
    ExecuteProcessListCommand,
    "proc_list_output.txt"
);

create_command_actor!(
    NetworkSpeedCommandActor,
    r#"
    devices=$(ip -o link show | awk -F': ' '{print $2}');
    for dev in $devices; do
        speed=$(ethtool $dev 2>/dev/null | grep "Speed" | awk '{print $2}');
        if [ -n "$speed" ]; then
            echo "Device: $dev, Speed: $speed";
        fi;
    done
    "#,
    ExecuteNetworkSpeedCommand,
    "network_speed_output.txt"
);

create_command_actor!(
    NetworkLoadCommandActor,
    r#"
    devices=$(netstat -i | awk 'NR>2 {print $1}' | grep -v ^lo);
    for device in $devices
    do
        rx_bytes=$(cat /sys/class/net/"$device"/statistics/rx_bytes);
        tx_bytes=$(cat /sys/class/net/"$device"/statistics/tx_bytes);
        rx_packets=$(cat /sys/class/net/"$device"/statistics/rx_packets);
        tx_packets=$(cat /sys/class/net/"$device"/statistics/tx_packets);

        echo "----- $device -----";
        echo "Received Bytes: $rx_bytes";
        echo "Transmitted Bytes: $tx_bytes";
        echo "Received Packets: $rx_packets";
        echo "Transmitted Packets: $tx_packets";
        echo;
    done
    "#,
    ExecuteNetworkLoadCommand,
    "network_load_output.txt"
);
