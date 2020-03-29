use std::fmt;
use std::ops::Deref;
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

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            size: 0,
        }
    }
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            pointer: self.head.as_ref().map(|boxed_lln| boxed_lln.deref()),
        }
    }
    pub fn push_front(&mut self, val: T) {
        let new_node = LinkedListNode {
            val,
            next: self.head.take(),
        };
        self.size += 1;
        self.head = Some(Box::new(new_node));
    }
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
    pub fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }
}
impl<T: PartialEq> LinkedList<T> {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        let mut count = 0;
        for val in self.iter() {
            if count < self.size - 1 {
                f.write_fmt(format_args!("{:?} . ", val))?;
            } else {
                f.write_fmt(format_args!("{:?}", val))?;
            }
            count += 1;
        }
        f.write_str(")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_test() {
        let mut ll = LinkedList::<i32>::new();
        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);
        assert_eq!(format!("{:?}", ll), "(3 . 2 . 1)");
    }

    #[test]
    fn sanity_pop() {
        let mut ll = LinkedList::<i32>::new();
        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);
        assert_eq!(3, ll.pop_front().unwrap());
    }

    #[test]
    fn sanity_contains() {
        let mut ll = LinkedList::<i32>::new();
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
        let mut ll = LinkedList::<i32>::new();
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
    let mut ll = LinkedList::<i32>::new();
    for i in 0..N {
        ll.push_front(i);
    }
    let guess = -1;
    b.iter(|| {
        ll.contains(&guess);
    })
}
