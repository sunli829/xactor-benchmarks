use std::time::{Duration, Instant};
use xactor::*;

struct Msg1(i64);

impl Message for Msg1 {
    type Result = i64;
}

struct Msg2;

impl Message for Msg2 {
    type Result = ();
}

struct MyActor(i64);

impl Actor for MyActor {}

#[async_trait::async_trait]
impl Handler<Msg1> for MyActor {
    async fn handle(&mut self, _ctx: &Context<Self>, msg: Msg1) -> i64 {
        self.0 += msg.0;
        self.0
    }
}

#[async_trait::async_trait]
impl Handler<Msg2> for MyActor {
    async fn handle(&mut self, _ctx: &Context<Self>, _msg: Msg2) {
        self.0 += 1;
    }
}

pub fn test() -> (Duration, Duration) {
    async_std::task::block_on(async {
        let mut addr = MyActor(0).start().await;

        let call_start = Instant::now();
        let mut sum = 0;
        for i in 0..100000 {
            sum += i;
            assert_eq!(sum, addr.call(Msg1(i)).await.unwrap());
        }
        let call_elapsed = call_start.elapsed();

        let send_start = Instant::now();
        for _ in 0..100000 {
            addr.send(Msg2).unwrap();
        }
        let send_elapsed = send_start.elapsed();

        (call_elapsed, send_elapsed)
    })
}
