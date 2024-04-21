use super::error::{Result, VmError};

/// The stack structure used by the VM.
/// The stack has a fixed capacity and can store any type.
pub struct Stack<T> {
    data: Vec<T>,
    capacity: usize,
}

impl<T> Stack<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, value: T) -> Result<()> {
        if self.data.len() == self.capacity {
            return Err(VmError::StackOverflow);
        }

        self.data.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T> {
        if self.data.is_empty() {
            return Err(VmError::StackUnderflow);
        }

        Ok(self.data.pop().unwrap())
    }

    pub fn peek(&self) -> Result<&T> {
        if self.data.is_empty() {
            return Err(VmError::StackUnderflow);
        }

        Ok(&self.data[self.data.len() - 1])
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_create() {
        let stack = Stack::<i32>::new(1024);
        assert_eq!(stack.capacity(), 1024);
    }

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::<i32>::new(1024);
        stack.push(42).unwrap();
        assert_eq!(stack.pop().unwrap(), 42);
    }

    #[test]
    fn test_stack_push_overflow() {
        let mut stack = Stack::<i32>::new(1);
        stack.push(42).unwrap();
        assert_eq!(stack.push(42).unwrap_err(), VmError::StackOverflow);
    }

    #[test]
    fn test_stack_pop_underflow() {
        let mut stack = Stack::<i32>::new(1);
        assert_eq!(stack.pop().unwrap_err(), VmError::StackUnderflow);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::<i32>::new(1024);
        stack.push(42).unwrap();
        assert_eq!(*stack.peek().unwrap(), 42);
    }

    #[test]
    fn test_stack_peek_underflow() {
        let stack = Stack::<i32>::new(1024);
        assert_eq!(stack.peek().unwrap_err(), VmError::StackUnderflow);
    }

    #[test]
    fn test_stack_len() {
        let mut stack = Stack::<i32>::new(1024);
        stack.push(42).unwrap();
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_stack_is_empty() {
        let stack = Stack::<i32>::new(1024);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_stack_clear() {
        let mut stack = Stack::<i32>::new(1024);
        stack.push(42).unwrap();
        stack.clear();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_stack_capacity() {
        let stack = Stack::<i32>::new(1024);
        assert_eq!(stack.capacity(), 1024);
    }
}
