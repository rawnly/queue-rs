use std::{collections::VecDeque, sync::Mutex};

use self::prelude::Queue;

pub mod prelude;

/// A FIFO queue implemented using a VecDeque and a Mutex
#[derive(Debug)]
pub struct FifoQueue<T> {
    /// The underlying data structure of the queue
    data: Mutex<VecDeque<T>>,
}

impl<T> Queue<T> for FifoQueue<T> {
    /// Creates a new, empty queue
    fn new() -> Self {
        Self {
            data: Mutex::new(VecDeque::new()),
        }
    }

    /// Adds an element to the back of the queue
    fn push(&self, value: T) {
        let mut data = self.data.lock().unwrap();
        data.push_back(value)
    }

    /// Removes an element from the front of the queue
    /// Returns None if the queue is empty
    fn pop(&self) -> Option<T> {
        let mut data = self.data.lock().unwrap();
        data.pop_front()
    }

    /// Returns the size of the queue
    fn size(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.len()
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use crate::FifoQueue;
    use std::{sync::Arc, thread};

    #[test]
    fn test_queue_thread_safety() {
        // createa a queue of numbers
        let queue = Arc::new(FifoQueue::<i32>::new());

        let q1 = queue.clone();
        let t1 = thread::spawn(move || {
            q1.push(1);
            q1.push(2);
        });

        let q2 = queue.clone();
        let t2 = thread::spawn(move || {
            q2.push(3);
            q2.push(4)
        });

        t1.join().unwrap();
        t2.join().unwrap();

        assert_eq!(queue.size(), 4);
    }
}

/// A NON-FIFO queue implemented using a VecDeque and a Mutex
#[derive(Debug)]
struct NonFifoQueue<T> {
    /// The underlying data structure of the queue
    data: Mutex<VecDeque<T>>,
}

impl<T> Queue<T> for NonFifoQueue<T> {
    /// Creates a new, empty queue
    fn new() -> Self {
        Self {
            data: Mutex::new(VecDeque::new()),
        }
    }

    /// Adds an element to the front of the queue
    fn push(&self, value: T) {
        let mut data = self.data.lock().unwrap();
        data.push_front(value)
    }

    /// Removes an element from the back of the queue
    /// Returns None if the queue is empty
    fn pop(&self) -> Option<T> {
        let mut data = self.data.lock().unwrap();
        data.pop_back()
    }

    /// Returns the size of the queue
    fn size(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.len()
    }
}
