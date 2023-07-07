use std::mem;

#[derive(Debug)]
pub enum FirstList<T> {
    Nil,
    // Always need to allocate a List for Next because Box always requires allocate instance
    // But compiler can optimize this case by using [null pointer optimization]
    // - Optimization only occured at first time when we create a Nil list
    // - It works like Option enum
    Next(T, Box<FirstList<T>>),
}

#[derive(Debug)]
pub enum SecondList<T> {
    Nil,
    // Unnecessary to allocate a list at the end
    // But add complexity to the code because there are more options in Enum
    // Even if it is empty, it is hard for the compiler to optimize
    Last(T),
    Next(T, Box<SecondList<T>>),
}

#[derive(Debug)]
struct Node {
    val: i32,
    // Compiler can optimize memory out if OptionList is Empty
    // It works like Option enum
    next: OptionList,
}

#[derive(Debug)]
enum OptionList {
    Empty,
    Populated(Box<Node>),
}

#[derive(Debug)]
pub struct BadStack {
    // Because we want to hide Node struct, so we can't let user access it
    // So we need a another wrapper struct and implement methods to handle encapsulation
    head: OptionList,
}

impl BadStack {
    pub fn new() -> Self {
        Self {
            head: OptionList::Empty,
        }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = OptionList::Populated(Box::new(Node {
            val,
            // mem::replace: replace first value to second value and return first value
            // Move head to a cache
            // Move OptionList::Empty to head
            // Move cache to next
            next: mem::replace(&mut self.head, OptionList::Empty),
        }));

        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<i32> {
        // We can't move (or assign) a instance while borrowing it
        // So we move that instance to a cache
        // Then replace that instance to a empty instance
        // And return the cache
        // So when we using returned cache, we no longer touch the instance
        match mem::replace(&mut self.head, OptionList::Empty) {
            OptionList::Populated(node) => {
                self.head = node.next;
                Some(node.val)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Still need to allocate a list at the end even if it is empty
        let list = FirstList::Next(1, Box::new(FirstList::Nil));
        println!("{:?}", list);

        let list = SecondList::Next(1, Box::new(SecondList::Last(2)));
        println!("{:?}", list);

        let mut list = BadStack::new();
        list.push(1);
        list.push(2);
        println!("{:?}", list);
        assert_eq!(list.pop(), Some(2));
    }
}
