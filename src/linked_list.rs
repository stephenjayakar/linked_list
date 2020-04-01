use std::fmt;
use std::ops::Deref;

#[derive(Default)]
pub struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
    size: usize,
}

struct LinkedListNode<T> {
    val: T,
    next: Option<Box<LinkedListNode<T>>>,
}

pub struct LinkedListIterator<'a, T> {
    pointer: Option<&'a LinkedListNode<T>>,
}

/// Singly linked-list data structure.
impl<T> LinkedList<T> {
    /// Returns an iterator over the structure.
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            pointer: self.head.as_ref().map(|boxed_lln| boxed_lln.deref()),
        }
    }
    /// Takes ownership of `val` and adds it to the front of the linked list in O(1).
    ///
    /// # Example
    /// ```
    /// use stephen_ll::LinkedList;
    /// let mut list: LinkedList<i32> = Default::default();
    /// list.push_front(2);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn push_front(&mut self, val: T) {
        let new_node = LinkedListNode {
            val,
            next: self.head.take(),
        };
        self.size += 1;
        self.head = Some(Box::new(new_node));
    }
    /// Returns and removes the value at the head of the list, or None if the list is empty.
    ///
    /// # Example
    /// ```
    /// use stephen_ll::LinkedList;
    /// let mut list: LinkedList<i32> = Default::default();
    /// list.push_front(7);
    /// assert_eq!(list.pop_front(), Some(7));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        let head = self.head.take()?;
        self.head = head.next;
        self.size -= 1;
        Some(head.val)
    }
    pub fn is_empty(&self) -> bool {
        self.len() != 0
    }
    pub fn len(&self) -> usize {
        self.size
    }
    /// Removes all values from the linked list, resetting it to its initial state.
    pub fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }
}

impl<T: PartialEq> LinkedList<T> {
    /// Checks if any values in the list are equal to `x`.  Linear runtime.
    pub fn contains(&self, x: &T) -> bool {
        self.iter().any(|e| e == x)
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.pointer {
            let ret_val = &p.val;
            self.pointer = p.next.as_ref().map(|boxed_lln| boxed_lln.deref());
            Some(ret_val)
        } else {
            None
        }
    }
}

impl<T> fmt::Debug for LinkedList<T>
    where T: fmt::Debug
{
    //! Here's how the debug implemention looks:
    //! ```
    //! use stephen_ll::LinkedList;
    //! let mut ll: LinkedList<i32> = Default::default();
    //! ll.push_front(1);
    //! ll.push_front(2);
    //! ll.push_front(3);
    //! assert_eq!(format!("{:?}", ll), "(3 . 2 . 1)");
    //! ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        for (count, val) in self.iter().enumerate() {
            if count < self.size - 1 {
                f.write_fmt(format_args!("{:?} . ", val))?;
            } else {
                f.write_fmt(format_args!("{:?}", val))?;
            }
        }
        f.write_str(")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_test() {
        let mut ll: LinkedList<i32> = Default::default();
        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);
        assert_eq!(format!("{:?}", ll), "(3 . 2 . 1)");
    }

    #[test]
    fn sanity_pop() {
        let mut ll: LinkedList<i32> = Default::default();
        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);
        assert_eq!(3, ll.pop_front().unwrap());
    }

    #[test]
    fn sanity_contains() {
        let mut ll: LinkedList<i32> = Default::default();
        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);
        assert!(ll.contains(&1));
        assert!(ll.contains(&2));
        assert!(ll.contains(&3));
        assert!(!ll.contains(&4));
    }

    // if there isn't anything in the list, the head should be None
    #[test]
    fn head_none_invariant() {
        let mut ll: LinkedList<i32> = Default::default();
        assert!(ll.head.is_none());
        ll.push_front(1);
        assert!(ll.head.is_some());
        ll.pop_front();
        assert!(ll.head.is_none());
        ll.push_front(2);
        ll.clear();
        assert!(ll.head.is_none());
    }
}

use test::Bencher;
use std::process::Termination;
const N: i32 = 10000;
#[bench]
fn contains(b: &mut Bencher) -> impl Termination {
    let mut ll: LinkedList<i32> = Default::default();
    for i in 0..N {
        ll.push_front(i);
    }
    let guess = -1;
    b.iter(|| {
        ll.contains(&guess);
    })
}
