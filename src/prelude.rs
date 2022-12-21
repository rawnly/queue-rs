/// A trait describing the general behaviour of a Queue
pub trait Queue<T> {
    /// Creates a new, empty queue
    fn new() -> Self;

    /// Push a new item in the queue
    fn push(&self, value: T);

    /// Removes an item from the queue
    fn pop(&self) -> T;

    /// Returns the size of the queue
    fn len(&self) -> usize;

    /// checks if the queue is empty
    fn is_empty(&self) -> bool;
}
