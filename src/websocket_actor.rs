use actix::*;  
use actix_web_actors::ws;  
use tokio::net::TcpStream;  
use tokio_tungstenite::WebSocketStream;  
  
pub struct WebSocketActor;  
  
impl WebSocketActor {  
    pub fn new() -> Self {  
        WebSocketActor  
    }  
}  
  
impl Actor for WebSocketActor {  
    type Context = ws::WebsocketContext<Self>;  
  
    fn started(&mut self, ctx: &mut Self::Context) {  
        println!("WebSocket actor started");  
    }  
}  
  
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {  
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {  
        match msg {  
            Ok(ws::Message::Text(text)) => {  
                ctx.text(text);  
            }  
            Ok(ws::Message::Binary(bin)) => {  
                ctx.binary(bin);  
            }  
            _ => (),  
        }  
    }  
}  
