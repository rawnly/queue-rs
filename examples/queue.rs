use queue_rs::prelude::*;
use queue_rs::FifoQueue;
use std::io::Write;
use std::time::Duration;
use std::{sync::Arc, thread};

fn flush() {
    let _ = std::io::stdout().flush();
}

fn main() {
    let queue = Arc::new(FifoQueue::<String>::new());
    let queue1 = Arc::clone(&queue);
    let queue2 = Arc::clone(&queue);

    let handle = thread::spawn(move || {
        print!("\n child thread started");
        flush();
        let dur = Duration::from_millis(20);

        loop {
            let t = queue1.pop();
            print!("\n dequeued {} on child thread", t);
            flush();

            if &t == "quit" {
                break;
            }

            thread::sleep(dur);
        }

        print!("\n child thread shutting down");
        flush();
    });

    let dur = Duration::from_millis(20);
    for i in 0..10 {
        let msg = format!("msg #{}", i);
        print!("\n enqueued {:?} on main thread", msg);
        flush();
        queue2.push(msg);
        thread::sleep(dur);
    }

    let msg = "quit".to_string();
    print!("\n enqueued {:?} on main thread", msg);
    flush();
    queue2.push(msg);

    print!("\n waiting for child process to stop");
    flush();
    handle.join().unwrap();
    print!("\n queue length = {}", queue2.len());
}
