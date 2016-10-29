extern crate zmq;

fn main() {

    let mut context = zmq::Context::new();
    let mut i = 0;
    loop {
        println!("entering {}", i);

        let mut responder = context.socket(zmq::REP).unwrap();
        println!("  {}", i);
        let res = responder.bind("tcp://0.0.0.0:5555");

        println!("bind result {:?}", res);

        assert!(res.is_ok());
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
        /*entering 0
  0
bind result Ok(())
entering 1
  1
bind result Ok(())
entering 2
  2
bind result Ok(())
entering 3
  3
bind result Ok(())
entering 4
  4
bind result Ok(())
entering 5
  5
bind result Ok(())
entering 6
  6
bind result Ok(())
entering 7
  7
bind result Err(Address already in use)
thread 'main' panicked at 'assertion failed: res.is_ok()', src/main.rs:16
note: Run with `RUST_BACKTRACE=1` for a backtrace.
        */


    }
}
