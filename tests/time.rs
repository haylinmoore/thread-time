extern crate thread_time;

use std::time::Duration;
use std::thread::sleep;
use std::thread;
use std::sync::mpsc::channel;

use thread_time::{ThreadTime};

// ThreadTime in our own thread
#[test]
fn thread_time_1s() {
    let time = ThreadTime::new();
    sleep(Duration::new(1, 0));
    let elapsed = time.elapsed();
    assert!(elapsed < Duration::from_millis(2));
}

// Wait on another thread while pinning our thread
#[test]
fn thread_time_between_threads_other_pinned() {
    let (tx, rx) = channel();
    thread::spawn(move|| {
        let time = ThreadTime::new();
        tx.send(time).unwrap();
        sleep(Duration::new(100, 0));
    });
    let time = rx.recv().unwrap();
    let mut _r: u64 = 0;
    for i in 0..100000000 {
        _r += i;
    }
    let elapsed = time.elapsed();
    assert!(elapsed < Duration::from_millis(2));
}

// Pin a thread for 250ms and see if it's clock is above 200ms
#[test]
fn thread_time_between_threads_main_pinned() {
    let (tx, rx) = channel();
    // Lets pin this thread
    let t = thread::spawn(move|| {
        let time = ThreadTime::new();
        tx.send(time).unwrap();
        loop {}
    });
    let time = rx.recv().unwrap();
    sleep(Duration::new(0, 250000000));
    let elapsed = time.elapsed();
    // I honestly don't know what proper tolerences are for testing this
    assert!(elapsed > Duration::from_millis(200));
    assert!(elapsed < Duration::from_millis(300));
}
