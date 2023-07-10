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
                        self.head = None;
                        None
                    }
                };

                self.len -= 1;
                old_tail.val
            })
        }
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &(*node.as_ptr()).val) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &(*node.as_ptr()).val) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut (*node.as_ptr()).val) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut (*node.as_ptr()).val) }
    }

    pub fn len(&self) -> usize {
        self.len
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
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len, Some(self.0.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.0.len
    }
}

struct Iter<'a, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.head.map(|node| {
            if self.len > 0 {
                self.len -= 1;
            }
            unsafe {
                self.head = (*node.as_ptr()).next;
                &(*node.as_ptr()).val
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tail.map(|node| {
            if self.len > 0 {
                self.len -= 1;
            }
            unsafe {
                self.tail = (*node.as_ptr()).prev;
                &(*node.as_ptr()).val
            }
        })
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

struct IterMut<'a, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.head.map(|node| {
            if self.len > 0 {
                self.len -= 1;
            }
            unsafe {
                self.head = (*node.as_ptr()).next;
                &mut (*node.as_ptr()).val
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tail.map(|node| {
            if self.len > 0 {
                self.len -= 1;
            }
            unsafe {
                self.tail = (*node.as_ptr()).prev;
                &mut (*node.as_ptr()).val
            }
        })
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _marker: PhantomData,
        }
    }
}

// IntoIterator auto deduced a List to a iterator
// for _ in list <=> for _ in list.into_iter()
impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

// for _ in &list <=> for _ in list.iter()
impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// for _ in &mut list <=> for _ in list.iter_mut()
impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("test a_production_unsafe_deque::first::tests::test");

        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
        if let Some(front) = list.front_mut() {
            *front += 10;
        }
        assert_eq!(list.pop_front(), Some(13));
        list.push_back(4);
        if let Some(back) = list.back_mut() {
            *back += 10;
        }
        assert_eq!(list.pop_back(), Some(14));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        list.push_front(3);
        list.push_front(2);
        list.push_front(4);
        list.push_front(1);
        list.push_back(5);

        for i in &mut list {
            *i += 10;
        }

        for i in &list {
            println!("{}", i);
        }

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(11));
        assert_eq!(iter.next_back(), Some(15));
        assert_eq!(iter.next_back(), Some(13));
        assert_eq!(iter.next(), Some(14));
        assert_eq!(iter.next(), Some(12));
    }
}
