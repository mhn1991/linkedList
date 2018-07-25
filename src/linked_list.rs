#[derive(Debug)]
pub struct LinkedList<T>{
    head: Option<Box<Node<T>>>,
    len: usize
}
#[derive(Debug)]
pub struct Node<T>{
    elem:T,
    next:Option<Box<Node<T>>>
}

impl<T> LinkedList<T>{
    pub fn new()->LinkedList<T>{
        LinkedList{
            head: None,
            len:0
        }
    }

    pub fn push(&mut self,element:T){
        let new_node=Box::new(Node{
            elem:element,
            next: self.head.take()
        });
        self.head = Some(new_node);
        self.len +=1;
    }

    pub fn pop(&mut self)->Option<T>{
        match self.head.take(){
            None=>None,
            Some(node)=>{
                let node = *node; // raw pointer
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

}
