use queue_rs::prelude::*;
use queue_rs::FifoQueue;
use std::time::Duration;
use std::{sync::Arc, thread};

fn main() {
    let queue = Arc::new(FifoQueue::<String>::new());
    let queue1 = queue.clone();
    let queue2 = queue.clone();

    let handle = thread::spawn(move || {
        println!("[CHILD_THREAD] Started");
        let dur = Duration::from_millis(20);

        loop {
            let t = queue1.pop();
            println!("[CHILD_THREAD] dequeued {}", t);

            if &t == "quit" {
                break;
            }

            thread::sleep(dur);
        }

        println!("[CHILD_THREAD] quitting");
    });

    let dur = Duration::from_millis(20);
    for i in 0..10 {
        let msg = format!("msg #{}", i);
        println!("[MAIN_THREAD] Enqueued {:?}", msg);
        queue2.push(msg);
        thread::sleep(dur);
    }

    let msg = "quit".to_string();
    println!("[MAIN_THREAD] Enqueued {:?}", msg);
    queue2.push(msg);

    println!("[MAIN_THREAD] Waiting for child process to stop");
    handle.join().unwrap();
}
