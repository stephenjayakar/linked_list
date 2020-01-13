mod linked_list;

use crate::linked_list::LinkedList;

fn main() {
    let mut ll = LinkedList::<i32>::new();
    ll.push(1);
    ll.push(2);
    ll.push(3);
    println!("{:?}", ll);
}