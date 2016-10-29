extern crate zmq;

fn main() {

    let mut context = zmq::Context::new();
    let mut i = 0;
    loop {
        println!("entering {}", i);

        let mut responder = context.socket(zmq::REP).unwrap();

        let linger_res = responder.set_linger(0);
        println!("linger result {:?}", linger_res);
        assert!(linger_res.is_ok());

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
linger result Ok(())
  0
bind result Ok(())
entering 1
linger result Ok(())
  1
bind result Ok(())
entering 2
linger result Ok(())
  2
bind result Ok(())
entering 3
linger result Ok(())
  3
bind result Err(Address already in use)
thread 'main' panicked at 'assertion failed: res.is_ok()', src/main.rs:21
note: Run with `RUST_BACKTRACE=1` for a backtrace.
*/


    }
}
