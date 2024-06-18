use actix::prelude::*;  
  
pub struct SupervisorActor;  
  
impl Actor for SupervisorActor {  
    type Context = Context<Self>;  
}  
  
impl Supervised for SupervisorActor {  
    fn restarting(&mut self, _: &mut Self::Context) {  
        println!("Supervisor is restarting");  
    }  
}  
  
impl SupervisorActor {  
    pub fn start_supervisor() {  
        let _ = SupervisorActor.start();  
    }  
}  
