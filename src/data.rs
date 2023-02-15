#[derive(Debug)]
pub(crate) struct Queue<T> { vec: Vec<T> }

impl<T> Queue<T> {
    pub fn new() -> Self { Queue { vec: Vec::new() } }

    pub fn len(&self) -> usize { self.vec.len() }

    pub fn mty(&self) -> bool { self.vec.is_empty() }

    pub fn peek(&self) -> Option<&T> { self.vec.first() }

    pub fn enq(&mut self, item: T) -> bool {self.vec.push(item); true}

    pub fn deq(&mut self) -> Option<T> {
        if self.mty() {None} 
        else { Some(self.vec.remove(0))}
    }
}

#[derive(Debug)]
pub(crate) struct Stack<T> { vec: Vec<T> }

impl<T> Stack<T> {
    pub fn new() -> Self { Stack { vec: Vec::new() } }

    pub fn len(&self) -> usize { self.vec.len() }

    pub fn mty(&self) -> bool { self.vec.is_empty() }

    pub fn peek(&self) -> Option<&T> { self.vec.last() }

    pub fn push(&mut self, item: T) -> bool { self.vec.push(item); true }

    pub fn pop(&mut self) -> Option<T> { self.vec.pop() }
}

#[cfg(test)]
mod tests {
    use super::Queue;
    use super::Stack;

    /// `queue_check` needs type `T` of `Queue` to check `==` and debug output `{:?}`.
    fn queue_check<T: PartialEq + std::fmt::Debug>(mut q: Queue<T>) -> bool {
        assert!(q.mty());
        assert_eq!(q.deq(), None);
        assert_eq!(q.peek(), None);
        assert_eq!(q.len(), 0);
        true
    }

    #[test]
    fn queue_mty() {
        let q: Queue<usize> = Queue::new();
        queue_check(q);
    }

    #[test]
    fn queue_ints() {
        let mut q = Queue::new();
        q.enq(3);
        q.enq(1);
        q.enq(4);
        q.enq(1);
        q.enq(5);

        assert_eq!(q.len(), 5);
        assert_eq!(q.deq().unwrap(), 3);
        assert_eq!(q.peek(), Some(&1));
        for _ in 1..=5 {
            q.deq();
        }
        queue_check(q);
    }

    #[test]
    fn queue_strs() {
        let mut q = Queue::new();

        q.enq("A");
        q.enq("B");
        assert_eq!(q.len(), 2);
        assert_eq!(q.peek(), Some(&"A"));

        q.deq();
        assert_eq!(q.len(), 1);
        assert_eq!(q.peek(), Some(&"B"));

        q.enq("C");
        assert_eq!(q.len(), 2);
        assert_eq!(q.peek(), Some(&"B"));

        q.deq();
        q.deq();
        queue_check(q);
    }

    /// `stack_check` needs type `T` of `Queue` to check `==` and debug output `{:?}`.
    fn stack_check<T: PartialEq + std::fmt::Debug>(mut s: Stack<T>) -> bool {
        assert!(s.mty());
        assert_eq!(s.pop(), None);
        assert_eq!(s.len(), 0);
        assert_eq!(s.peek(), None);
        true
    }

    #[test]
    fn stack_mty() {
        let s: Stack<String> = Stack::new();
        stack_check(s);
    }

    #[test]
    fn stack_ints() {
        let mut s = Stack::new();

        s.push(3);
        s.push(1);
        s.push(4);
        s.push(1);
        s.push(5);

        assert!(!s.mty());
        assert_eq!(s.len(), 5);
        assert_eq!(s.peek(), Some(&5));
        for _ in 1..=5 {
            s.pop();
        }
        stack_check(s);
    }

    #[test]
    fn stack_strs() {
        let mut s = Stack::new();

        s.push("A");
        assert_eq!(s.len(), 1);
        assert_eq!(s.peek(), Some(&"A"));

        s.push("B");
        assert_eq!(s.len(), 2);
        assert_eq!(s.peek(), Some(&"B"));
        
        s.pop();
        s.pop();
        stack_check(s);
    }
}
