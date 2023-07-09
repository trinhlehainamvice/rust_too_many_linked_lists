#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct List<'a, T> {
    head: Option<Box<Node<T>>>,
    // When we first push, tail will reference Node inside head
    // Which mean element reference or borrow another element inside a List
    // Mean when we push, a List will borrow itself
    tail: Option<&'a mut Node<T>>,
    // This situation is desirable because Rust avoid us to modify another element while an element is borrowed
    // For example, if tail reference to Node inside head after we first push
    // It's unsafe to assign or move head, cause tail reference to moved instance
}

impl<'a, T> List<'a, T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    // If we don't explicitly assign lifetime to &mut self, but we declare a lifetime in impl 'a
    // Rust compiler will need to confirm that the lifetime of self is 'a or outlives 'a
    pub fn push(&'a mut self, val: T) {
        let new_node = Box::new(Node { val, next: None });

        let new_tail = match self.tail.take() {
            Some(old_tail) => {
                old_tail.next = Some(new_node);
                old_tail.next.as_deref_mut()
            }
            _ => {
                self.head = Some(new_node);
                self.head.as_deref_mut()
            }
        };

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            if node.next.is_none() {
                self.tail = None;
            }
            self.head = node.next;
            node.val
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
    }
}
