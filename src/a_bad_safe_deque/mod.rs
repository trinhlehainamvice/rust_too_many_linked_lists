use std::cell::{Ref, RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;

struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

struct List<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, val: T) {
        let new_head = Rc::new(RefCell::new(Node {
            val,
            next: None,
            prev: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }

        self.len += 1;
    }

    pub fn push_back(&mut self, val: T) {
        let new_tail = Rc::new(RefCell::new(Node {
            val,
            next: None,
            prev: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            _ => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                _ => {
                    self.tail.take();
                }
            }
            self.len -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                _ => {
                    self.head.take();
                }
            }

            self.len -= 1;
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val
        })
    }

    pub fn front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            // borrowed value by RefCell::borrow() return a Ref<'_, T> which has a same lifetime with owner
            // so can't reference to borrowed value if that's owner is dropped in this scope
            // mean same case with return a reference to data created inside a function or closure
            // .map(|node| &node.borrow().val)
            // TODO: need more research
            .map(|node| Ref::map(node.borrow(), |node| &node.val))
    }

    pub fn back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.val))
    }

    pub fn front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.val))
    }

    pub fn back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.val))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

/* struct Iter<T>(Option<Rc<RefCell<Node<T>>>>);
   With Rc version, we can't define lifetime of Iter
   So we can't get Ref or RefMut of RefCell if we can't define lifetime
*/

// TODO: research
/* struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);
   With Ref version, even though we can define lifetime
   We still can't handle borrow properly
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(*list.front().unwrap(), 3);
        assert_eq!(*list.back().unwrap(), 1);
        list.push_back(4);
        *list.back_mut().unwrap() = 5;
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(*list.back().unwrap(), 1);
        assert_eq!(*list.front().unwrap(), 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(*list.front().unwrap(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
    }
}
