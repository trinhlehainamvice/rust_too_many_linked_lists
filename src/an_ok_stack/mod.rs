#[derive(Debug)]
struct Node<T> {
    val: T,
    // Compiler can optimize memory out if OptionList is Empty
    // It works like Option enum
    next: OptionList<T>,
}

// In BadStack version, we reinvent the built-in Option enum
// Let use Option enum instead of reinvent it
type OptionList<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct OkStack<T> {
    // Because we want to hide Node struct, so we can't let user access it
    // So we need a another wrapper struct and implement methods to handle encapsulation
    head: OptionList<T>,
}

impl<T> OkStack<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, val: T) {
        let new_node = Some(Box::new(Node {
            val,
            // mem::replace: replace first value to second value and return first value
            // Move head to a cache
            // Move OptionList::Empty to head
            // Move cache to next
            next: self.head.take(),
        }));

        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // Below code moves head to head to closure inside map
        // -> node will be dropped after this method is cleared on stack
        // -> reference to node cause dangling pointer
        // self.head.map(|node| &node.val)
        // Instead reference to head first before pass to closure
        // -> node right now is a reference to head [or borrow head]
        // -> so even this method is cleared on stack, node value still exist [or head is not moved or dropped]
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut list = OkStack::new();
        list.push(1);
        list.push(2);
        println!("{:?}", list);
        assert_eq!(list.peek(), Some(&2));
        *list.peek_mut().unwrap() += 1;
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
