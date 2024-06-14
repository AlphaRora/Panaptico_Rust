// websocket_actor.rs
use actix::prelude::*;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use futures_util::StreamExt;

pub struct WebSocketActor {
    ws_stream: WebSocketStream<TcpStream>,
}

impl WebSocketActor {
    pub fn new(ws_stream: WebSocketStream<TcpStream>) -> Self {
        WebSocketActor { ws_stream }
    }
}

impl Actor for WebSocketActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.add_stream(self.ws_stream.take());
    }
}

impl StreamHandler<Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>> for WebSocketActor {
    fn handle(&mut self, msg: Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>, ctx: &mut Self::Context) {
        match msg {
            Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                // Handle incoming WebSocket messages here
                println!("Received WebSocket message: {}", text);
            }
            _ => {}
        }
    }
}
