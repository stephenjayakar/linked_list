use std::fmt;
pub struct LinkedList<T> {
    head: Option<LinkedListNode<T>>,
    size: usize,
}

struct LinkedListNode<T> {
    val: T,
    next: Box<Option<LinkedListNode<T>>>,
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
            pointer: self.head.as_ref(),
        }
    }
    pub fn push(&mut self, val: T) {
        let new_node = LinkedListNode {
            val,
            next: Box::new(None),
        };
        self.size += 1;
        match &mut self.head {
            None => self.head = Some(new_node),
            Some(node) => {
                let mut pointer = node;
                while let Some(ref mut p) = *pointer.next {
                    pointer = p;
                }
                pointer.next = Box::new(Some(new_node));
            }
        }
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.pointer {
            let ret_val = &p.val;
            self.pointer = (*p.next).as_ref();
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