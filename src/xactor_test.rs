use super::{Result as BenchResult, Spec};
use std::time::{Duration, Instant};
use xactor::*;

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

#[message(result = "()")]
#[derive(Clone)]
struct Data(String);

// The close ring message to set the first actor as next for the last actor
#[message(result = "()")]
struct CloseRing {
    first: Addr<RingActor>,
}

// Actor implementation
#[async_trait::async_trait]
impl Actor for RingActor {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
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
                .start()
                .await
                .unwrap(),
            );
        };

        Ok(())
    }
}

// Implementation of the handler for Data message
#[async_trait::async_trait]
impl Handler<Data> for RingActor {
    async fn handle(&mut self, ctx: &Context<Self>, msg: Data) -> () {
        match self.next {
            Some(ref mut next) => {
                // If we do we check if we are the first process
                if self.id == 0 {
                    // If so we know we can end our system if we have send our
                    // messages
                    if self.msgs == 0 {
                        ctx.stop(None);
                    } else {
                        // Otherwise we decrement the message count and send a new
                        // `Data` message to the next actor in the ring.
                        self.msgs -= 1;
                        next.send(msg.clone());
                    }
                } else {
                    // If we are not the first process we just keep passing on
                    // `Data` messages
                    next.send(msg.clone());
                }
            }
            // so if it does we panic!
            None => panic!(
                "[{}] Next was null! This is not a ring it's a string :(",
                self.id
            ),
        };
    }
}

#[async_trait::async_trait]
impl Handler<CloseRing> for RingActor {
    async fn handle(&mut self, _ctx: &Context<Self>, msg: CloseRing) -> () {
        match self.next {
            // If we have a next we pass this message on to the next actor
            Some(ref mut next) => {
                next.send(msg);
                ()
            }
            // If not weŕe the last actor and set the first node as our next node
            None => {
                self.next = Some(msg.first);
                ()
            }
        };
    }
}

pub async fn run(spec: &Spec) -> BenchResult {
    // Pre-generate the payload so weŕe not measuring string
    // creation.
    let data = (0..spec.size).map(|_| "x").collect::<String>();

    // Create our first actor
    let mut addr: Addr<_> = RingActor {
        next: None,
        id: 0,
        max: spec.procs,
        msgs: spec.messages,
    }
    .start()
    .await
    .unwrap();

    // Since the first actor will create the second and so one at this
    // point our ring is nearly complete it just needs to be closed.
    addr.send(CloseRing {
        first: addr.clone(),
    });

    // Next we put Data messages on the ring limited by the number
    // of parallel messages we want.
    for _ in 0..spec.parallel {
        addr.send(Data(data.clone()));
    }

    // The ring will run until our first actor decides it's time to shut down.
    BenchResult {
        name: String::from("rust_xactor"),
        spec: spec.clone(),
    }
}
