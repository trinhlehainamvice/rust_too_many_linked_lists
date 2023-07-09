struct Node<T> {
    val: T,
    next: *mut Node<T>,
}

struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
        }
    }

    pub fn push(&mut self, val: T) {
        // Need Box::new to allocate Node in heap
        let new_node = Box::into_raw(Box::new(Node {
            val,
            next: std::ptr::null_mut(),
        }));

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = new_node;
            }
        } else {
            self.head = new_node;
        }

        self.tail = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.head.is_null() {
            unsafe {
                // own raw_pointer with Box::from_raw, old_head will be dropped when go out of unsafe scope
                let old_head = Box::from_raw(self.head);
                self.head = old_head.next;
                Some(old_head.val)
            }
        } else {
            self.tail = std::ptr::null_mut();
            None
        }
    }

    pub fn head(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.val) }
    }

    pub fn tail(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.val) }
    }

    pub fn head_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.val) }
    }

    pub fn tail_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.val) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            /*
            self.0 = match node.next.is_null() {
                true => None,
                false => unsafe { Some(&*node.next) },
            };
            */

            unsafe {
                // Equivalent to above code
                // See inside as_ref for more details
                self.0 = node.next.as_ref();
            }

            &node.val
        })
    }
}

struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            /*
            self.0 = match node.next.is_null() {
                true => None,
                false => unsafe { Some(&mut *node.next) },
            };
            */

            unsafe {
                // Equivalent to above code
                // See inside as_mut for more details
                self.0 = node.next.as_mut();
            }

            &mut node.val
        })
    }
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        /*
        Iter(match self.head.is_null() {
            true => None,
            false => unsafe { Some(&*self.head) },
        })
        */
        unsafe { Iter(self.head.as_ref()) }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        /*
        IterMut(match self.head.is_null() {
            true => None,
            false => unsafe { Some(&mut *self.head) },
        })
        */
        unsafe { IterMut(self.head.as_mut()) }
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
        for i in list.iter() {
            println!("{}", i);
        }
        for i in list.iter_mut() {
            *i += 10;
        }
        assert_eq!(list.pop(), Some(11));
        assert_eq!(list.pop(), Some(12));
        assert_eq!(list.pop(), Some(13));
        assert_eq!(list.pop(), None);
    }
}
