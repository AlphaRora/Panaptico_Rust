use actix::prelude::*;
use actix::{Actor, Context, Supervised, Supervisor, System};

pub struct SupervisorActor;

impl Actor for SupervisorActor {
    type Context = Context<Self>;
}

impl Supervised for SupervisorActor {
    fn restarting(&mut self, _: &mut Self::Context) {
        println!("Supervisor is restarting");
    }
}

impl Supervisor for SupervisorActor {}

impl SupervisorActor {
    pub fn start_supervisor() {
        let _ = Supervisor::start(move |_| SupervisorActor);
    }
}
