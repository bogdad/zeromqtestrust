extern crate zmq;

fn main() {

    let mut context = zmq::Context::new();
    let mut i = 0;
    loop {
        println!("entering {}", i);

        let mut responder = context.socket(zmq::REP).unwrap();
        println!("  {}", i);

        assert!(responder.bind("tcp://*:5555").is_ok());
        i+=1;

        // if yod dont unbind you get this:
        // vshakhov$ target/debug/zeromqtestrust
        // entering 0
        // 0
        // entering 1
        // 1
        // thread 'main' panicked at 'assertion failed: responder.bind("tcp://*:5555").is_ok()', src/main.rs:21
        // note: Run with `RUST_BACKTRACE=1` for a backtrace.
        responder.unbind("tcp://*:5555").is_ok();

    }
}
