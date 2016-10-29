extern crate zmq;

fn main() {

    let mut context = zmq::Context::new();
    let mut i = 0;
    loop {
        println!("entering {}", i);

        let mut responder = context.socket(zmq::REP).unwrap();
        println!("  {}", i);

        assert!(responder.bind("tcp://0.0.0.0:5555").is_ok());
        i+=1;

        let result = responder.unbind("tcp://0.0.0.0:5555");
        assert!(result.is_ok());

        // if yod dont unbind you shorter number of iterations:
        // vshakhov$ target/debug/zeromqtestrust
        // entering 0
        // 0
        // entering 1
        // 1
        // thread 'main' panicked at 'assertion failed: responder.bind("tcp://*:5555").is_ok()', src/main.rs:21
        // note: Run with `RUST_BACKTRACE=1` for a backtrace.

        // vs with unbind
        /*
        entering 0
  0
entering 1
  1
entering 2
  2
entering 3
  3
entering 4
  4
entering 5
  5
entering 6
  6
entering 7
  7
entering 8
  8
entering 9
  9
entering 10
  10
entering 11
  11
entering 12
  12
entering 13
  13
entering 14
  14
entering 15
  15
entering 16
  16
entering 17
  17
entering 18
  18
entering 19
  19
entering 20
  20
entering 21
  21
entering 22
  22
entering 23
  23
entering 24
  24
thread 'main' panicked at 'assertion failed: responder.bind("tcp://0.0.0.0:5555").is_ok()', src/main.rs:13
note: Run with `RUST_BACKTRACE=1` for a backtrace.
        */


    }
}
