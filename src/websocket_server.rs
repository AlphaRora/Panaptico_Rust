use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream};

pub async fn start_websocket_server(addr: SocketAddr) {
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");
    println!("WebSocket server listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

async fn accept_connection(stream: TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Failed to accept WebSocket connection");

    process_websocket_connection(ws_stream).await;
}

async fn process_websocket_connection(mut ws_stream: WebSocketStream<TcpStream>) {
    while let Some(msg) = ws_stream.next().await {
        let msg = msg.expect("Failed to receive WebSocket message");
        if msg.is_binary() || msg.is_text() {
            // Handle incoming WebSocket messages here
            // You can call your existing command_executor functions
            // or perform other actions based on the received message
            println!("Received WebSocket message: {:?}", msg);
        }
    }
}