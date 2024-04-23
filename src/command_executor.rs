// command_executor.rs
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::io::{BufRead, BufReader};
use std::thread;

fn execute_bash_command(tx: Sender<String>, lscpu_tx: Sender<String>, rx: Receiver<bool>) {
    let mut lscpu_child = Command::new("lscpu")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn lscpu child process");

    let lscpu_stdout = lscpu_child.stdout.take().expect("Failed to get lscpu child stdout");
    let lscpu_stdout_reader = BufReader::new(lscpu_stdout);

    for line in lscpu_stdout_reader.lines() {
        match line {
            Ok(lscpu_output) => {
                if let Err(e) = lscpu_tx.send(lscpu_output) {
                    eprintln!("Failed to send lscpu command output: {}", e);
                    break;
                }
            }
            Err(e) => eprintln!("Failed to read line from lscpu child stdout: {}", e),
        }

        // Check if the receiver is still active
        match rx.recv() {
            Ok(_) => continue,
            Err(e) => {
                eprintln!("Error receiving command output: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let (tx, rx) = channel();
    let (lscpu_tx, lscpu_rx) = channel();

    let handle = thread::spawn(move || {
        execute_bash_command(tx, lscpu_tx, rx);
    });

    // Keep the receiver active until you're done processing the data
    // ...

    // Drop the receiver to signal the sender to stop
    drop(lscpu_rx);

    handle.join().unwrap();
}