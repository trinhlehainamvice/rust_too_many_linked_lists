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

// Consume Iterator, mean we can't use OkStack after convert into Iterator
// Because IntoIter will own OkStack instance.
pub struct IntoIter<T>(OkStack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
    // Prefer &T rather than &Box<T>
    // Because we can use deref() on &Box<T> to get &T
    // Reference: https://rust-lang.github.io/rust-clippy/master/index.html#/borrowed_box
    // next: Option<&'a Box<Node<T>>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            // We can write as below by explicitly declare turbofish type [::<U, _>] that we want to transform to
            // self.next = node.next.as_ref().map::<&Node<T>, _>(|node| node);
            &node.val
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.val
        })
    }
}

impl<T> OkStack<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // pub fn iter<'a>(&'a self) -> Iter<'a, T>
    // lifetime elision will automatically detect how to match lifetime in certain cases
    // so we don't need to write lifetime parameter like above case to more readable
    // Reference: https://rust-lang.github.io/rust-clippy/master/index.html#/needless_lifetimes
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
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

        list.push(0);
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), None);
        println!("{:?}", list);

        for next in list.iter_mut() {
            *next += 1;
        }
        println!("{:?}", list);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
