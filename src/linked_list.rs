use std::fmt;
use std::ops::Deref;
const CHUNK_SIZE: usize = 4;

pub struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
    size: usize,
}

struct LinkedListNode<T> {
    // for push_front, is filled from back -> front
    vals: [Option<T>; CHUNK_SIZE],
    next: Option<Box<LinkedListNode<T>>>,
}

impl<T> LinkedListNode<T> {
    fn new() -> LinkedListNode<T> {
        LinkedListNode {
            // TODO: See a better way to get around this
            vals: [None, None, None, None],
            next: None,
        }
    }
    fn next_free_index(&self) -> Option<usize> {
        let mut index = 0;
        let mut pointer = &self.vals[index];
        while pointer.is_none() && index < CHUNK_SIZE {
            index += 1;
            pointer = &self.vals[index];
        }
        match index {
            // Case where there isn't free space in the block
            0 => None,
            _ => Some(index - 1),
        }
    }
    fn is_empty(&self) -> bool {
        self.vals[CHUNK_SIZE - 1].is_none()
    }
}

impl<T: fmt::Debug> fmt::Debug for LinkedListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[")?;
        for i in 0..CHUNK_SIZE {
            match &self.vals[i] {
                None => f.write_str(" . ")?,
                Some(val) => f.write_fmt(format_args!(" {:?} ", val))?
            };
        }
        f.write_str("]")
    }
}

pub struct LinkedListIterator<'a, T> {
    pointer: Option<&'a LinkedListNode<T>>,
    chunk_offset: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            size: 0,
        }
    }
    pub fn iter(&self) -> LinkedListIterator<T> {
        let chunk_offset = if self.head.is_none() {
            0
        } else {
            match self.head.as_ref().unwrap().next_free_index() {
                None => 0,
                Some(i) => i + 1,
            }
        };
        LinkedListIterator {
            pointer: self.head.as_ref().map(|boxed_lln| boxed_lln.deref()),
            chunk_offset,
        }
    }
    pub fn push_front(&mut self, val: T) {
        self.size += 1;
        // case where the list is empty
        if self.len() == 1 {
            let mut new_node = LinkedListNode::new();
            new_node.vals[CHUNK_SIZE - 1] = Some(val);
            self.head = Some(Box::new(new_node));
            return;
        }

        let mut head = self.head.take().unwrap();
        match head.next_free_index() {
            Some(i) => {
                head.vals[i] = Some(val);
                self.head = Some(head);
            }
            None => {
                let mut new_node = LinkedListNode::new();
                // TODO: move all array accesses into LinkedListNode
                new_node.vals[CHUNK_SIZE - 1] = Some(val);
                new_node.next = Some(head);
                self.head = Some(Box::new(new_node));
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        let mut head = self.head.take()?;
        let index_to_pop = match head.next_free_index() {
            None => 0,
            Some(i) => i + 1,
        };
        let val = head.vals[index_to_pop].take();
        if head.is_empty() {
            self.head = head.next;
        } else {
            self.head = Some(head);
        }
        self.size -= 1;
        val
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
            let ret_val = p.vals[self.chunk_offset].as_ref().unwrap();
            self.chunk_offset += 1;
            if self.chunk_offset == CHUNK_SIZE {
                self.chunk_offset = 0;
                self.pointer = p.next.as_ref().map(|boxed_lln| boxed_lln.deref());
            }
            Some(ret_val)
        } else {
            None
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pointer = &self.head;
        f.write_str("(")?;
        while !pointer.is_none() {
            f.write_fmt(format_args!("{:?}", pointer.as_ref().unwrap()))?;
            pointer = &pointer.as_ref().unwrap().next;
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
        for i in 1..8 {
            ll.push_front(i);
        }
        println!("{:?}", ll);
        assert_eq!(format!("{:?}", ll), "([ .  7  6  5 ][ 4  3  2  1 ])");
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
}

use std::process::Termination;
use test::Bencher;
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
