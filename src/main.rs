use std::time::Duration;

mod actix_test;
mod xactor_test;

fn main() {
    test_it("actix", actix_test::test);
    test_it("xactor", xactor_test::test);
}

fn test_it<F: Fn() -> (Duration, Duration)>(name: &str, f: F) {
    let (call_elapsed, send_elapsed) = f();
    println!(
        "{}| Wait for response: {} ms\tOnly send: {} ms",
        name,
        call_elapsed.as_millis(),
        send_elapsed.as_millis()
    );
}
