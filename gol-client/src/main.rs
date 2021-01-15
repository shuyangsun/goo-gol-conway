use std::thread;
use std::time::Duration;

/// This is mainly just for concurrency/parallelism practice for now.
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("*** {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
