use super::{Result as BenchResult, Spec};
use actix::{prelude::*, *};
use std::time::{Duration, Instant};

// The ring actor
struct RingActor {
    // Next actor in the ring - must allow None as the actor only knows
    // about the next actor once it has been created.
    next: Option<Addr<RingActor>>,
    // The actor id (place in ring)
    id: u32,
    // Max number of actors we want
    max: u32,
    // Number of messages to pass
    msgs: u32,
}

#[derive(Clone)]
struct Data(String);
impl Message for Data {
    type Result = bool;
}
// The close ring message to set the first actor as next for the last actor
struct CloseRing {
    first: Addr<RingActor>,
}
impl Message for CloseRing {
    type Result = bool;
}

// Actor implementation
impl Actor for RingActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // As long as the actor id is smaller then the max
        // number of actors create a new actor with the next id
        if self.id < self.max {
            self.next = Some(
                RingActor {
                    next: None,
                    id: self.id + 1,
                    max: self.max,
                    msgs: self.msgs,
                }
                .start(),
            );
        };
    }
}

// Implementation of the handler for Data message
impl Handler<Data> for RingActor {
    type Result = bool;
    fn handle(&mut self, msg: Data, _ctx: &mut Context<Self>) -> Self::Result {
        match self.next {
            Some(ref next) => {
                // If we do we check if we are the first process
                if self.id == 0 {
                    // If so we know we can end our system if we have send our
                    // messages
                    if self.msgs == 0 {
                        System::current().stop();
                    } else {
                        // Otherwise we decrement the message count and send a new
                        // `Data` message to the next actor in the ring.
                        self.msgs -= 1;
                        next.do_send(msg.clone());
                    }
                } else {
                    // If we are not the first process we just keep passing on
                    // `Data` messages
                    next.do_send(msg.clone());
                }
            }
            // so if it does we panic!
            None => panic!(
                "[{}] Next was null! This is not a ring it's a string :(",
                self.id
            ),
        };
        true
    }
}
impl Handler<CloseRing> for RingActor {
    type Result = bool;
    fn handle(&mut self, msg: CloseRing, _ctx: &mut Context<Self>) -> Self::Result {
        match self.next {
            // If we have a next we pass this message on to the next actor
            Some(ref next) => {
                next.do_send(msg);
                ()
            }
            // If not weŕe the last actor and set the first node as our next node
            None => {
                self.next = Some(msg.first);
                ()
            }
        };
        true
    }
}

// TODO: handle multiple arbiters across multiple cores/threads
pub fn run(spec: &Spec) -> BenchResult {
    // Pre-generate the payload so weŕe not measuring string
    // creation.
    let data = (0..spec.size).map(|_| "x").collect::<String>();

    // Start a new System.
    let system = System::new("bench");
    // Create our first actor
    let addr: Addr<_> = RingActor {
        next: None,
        id: 0,
        max: spec.procs,
        msgs: spec.messages,
    }
    .start();

    // Since the first actor will create the second and so one at this
    // point our ring is nearly complete it just needs to be closed.
    addr.do_send(CloseRing {
        first: addr.clone(),
    });

    // Next we put Data messages on the ring limited by the number
    // of parallel messages we want.
    for _ in 0..spec.parallel {
        addr.do_send(Data(data.clone()));
    }

    // The ring will run until our first actor decides it's time to shut down.
    system.run();
    BenchResult {
        name: String::from("rust_actix"),
        spec: spec.clone(),
    }
}
