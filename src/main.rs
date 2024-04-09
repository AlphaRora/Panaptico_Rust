mod command_executor;
mod websocket_server;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Failed to parse socket address");
    websocket_server::start_websocket_server(addr).await;
}
