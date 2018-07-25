mod linked_list;
use linked_list::LinkedList;
fn main() {
    let mut ll:LinkedList<i32> = LinkedList::new();
    ll.push(3);
    println!("{:?}",ll.pop());
    println!("{:?}",ll.pop());
}
