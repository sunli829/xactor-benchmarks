use actix::prelude::*;
use std::time::{Duration, Instant};

struct Msg1(i64);

impl Message for Msg1 {
    type Result = i64;
}

struct Msg2;

impl Message for Msg2 {
    type Result = ();
}

struct MyActor(i64);

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<Msg1> for MyActor {
    type Result = i64;

    fn handle(&mut self, msg: Msg1, _ctx: &mut Self::Context) -> i64 {
        self.0 += msg.0;
        self.0
    }
}

impl Handler<Msg2> for MyActor {
    type Result = ();

    fn handle(&mut self, _msg: Msg2, _ctx: &mut Self::Context) {
        self.0 += 1;
    }
}

pub fn test() -> (Duration, Duration) {
    actix_rt::System::new("test").block_on(async {
        let addr = MyActor(0).start();

        let call_start = Instant::now();
        let mut sum = 0;
        for i in 0..100000 {
            sum += i;
            assert_eq!(sum, addr.send(Msg1(i)).await.unwrap());
        }
        let call_elapsed = call_start.elapsed();

        let send_start = Instant::now();
        for _ in 0..100000 {
            addr.do_send(Msg2);
        }
        let send_elapsed = send_start.elapsed();

        (call_elapsed, send_elapsed)
    })
}
