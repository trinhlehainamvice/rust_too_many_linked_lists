struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

struct List<T> {
    head: Option<Box<Node<T>>>,
    // raw pointer
    tail: *mut Node<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    pub fn push(&mut self, val: T) {
        let mut new_node = Box::new(Node { val, next: None });

        let new_tail: *mut _ = &mut *new_node;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_node);
            }
        } else {
            self.head = Some(new_node);
        }

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            if old_head.next.is_none() {
                self.tail = std::ptr::null_mut();
            }
            self.head = old_head.next;
            old_head.val
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }
}
