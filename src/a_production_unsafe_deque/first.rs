use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

struct List<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn push_front(&mut self, val: T) {
        unsafe {
            let new_head = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                val,
                next: None,
                prev: None,
            })));

            if let Some(old_head) = self.head {
                (*old_head.as_ptr()).prev = Some(new_head);
                (*new_head.as_ptr()).next = Some(old_head);
            } else {
                debug_assert!(self.tail.is_none());
                debug_assert!(self.head.is_none());
                debug_assert!(self.len == 0);
                self.tail = Some(new_head);
            }

            self.head = Some(new_head);
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, val: T) {
        unsafe {
            let new_tail = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                val,
                next: None,
                prev: None,
            })));

            if let Some(old_tail) = self.tail {
                (*old_tail.as_ptr()).next = Some(new_tail);
                (*new_tail.as_ptr()).prev = Some(old_tail);
            } else {
                debug_assert!(self.head.is_none());
                debug_assert!(self.tail.is_none());
                debug_assert!(self.len == 0);
                self.head = Some(new_tail);
            }

            self.tail = Some(new_tail);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.head.map(|old_head| {
                let old_head = Box::from_raw(old_head.as_ptr());

                self.head = match old_head.next {
                    Some(new_head) => {
                        (*new_head.as_ptr()).prev = None;
                        Some(new_head)
                    }
                    None => {
                        debug_assert!(self.len == 1);
                        self.tail = None;
                        None
                    }
                };

                self.len -= 1;
                old_head.val
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.tail.map(|old_tail| {
                let old_tail = Box::from_raw(old_tail.as_ptr());

                self.tail = match old_tail.prev {
                    Some(new_tail) => {
                        (*new_tail.as_ptr()).next = None;
                        Some(new_tail)
                    }
                    None => {
                        debug_assert!(self.len == 1);
                        self.head = None;
                        None
                    }
                };

                self.len -= 1;
                old_tail.val
            })
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3));
        list.push_back(4);
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }
}
