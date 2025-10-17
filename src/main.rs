use core::hash;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Mutex};


fn main() {
   println!("Hello, world!");

   
}


struct Node{
    hash_value: u64,
    resources: HashMap<u64, u64>,
    next: Option<NodeRef>,
    previous: Option<NodeRef>,
}

type NodeRef = Rc<RefCell<Node>>;

impl Node{
    fn new(hash_value: u64) -> Self{
        Self { hash_value, resources: HashMap::new(), next: None, previous: None }
    }

    fn next(&self) -> NodeRef{
        self.next.as_ref().unwrap().clone()
    }
}

struct HashRing{
    head: Option<NodeRef>,
    k: u32,
    min: u64,
    max: u64,
}

impl HashRing{
    fn new(k: u32) -> Self{
        Self { head: None, k, min: 0, max: 2u64.pow(k) - 1 }
    }

    fn head(&self) -> NodeRef{
        // in initial state, head is itself.
        self.head.as_ref().unwrap().clone()
    }

    fn is_in_legal_range(&self, hash_value: u64) -> bool {
        hash_value >= self.min && hash_value <= self.max
    }

    fn distance(&self, a: u64, b: u64) -> u64{
        if a == b {
            0
        } else if a < b {
            b - a
        } else {
            2u64.pow(self.k) + b - a
        }
    }

    fn lookup_node_mut(&mut self, hash_value: u64) -> NodeRef{
        if self.is_in_legal_range(hash_value){
            let mut temp = self.head();
            let next = temp.borrow().next();

            while self.distance(temp.borrow().hash_value, hash_value) >
                self.distance(next.borrow().hash_value, hash_value){
                    // temp = temp.nesxt;
                    temp = next.clone();
                    if temp.borrow().hash_value == hash_value{
                        return temp
                    }
            }
            return next;
        }
        panic!("Hash value out of range");
    }

    fn move_resources(&mut self, dest: &mut Node, orig: &mut Node, delete_true: bool){
        let mut delete_list = vec![];
        for (i, j) in orig.resources.iter(){
            if self.distance(*i, dest.hash_value) < self.distance(*i, orig.hash_value) || delete_true{
                dest.resources.insert(*i, *j);
                delete_list.push(*i);
            }
        }

        for i in delete_list.iter(){
            orig.resources.remove(i);
        }
    }

    fn add_node(&mut self, hash_value: u64){
        if self.is_in_legal_range(hash_value){
            let new_node = Rc::new(RefCell::new(Node::new(hash_value)));

            if self.head.is_none(){
                new_node.as_ref().borrow_mut().next = Some(new_node.clone());
                new_node.as_ref().borrow_mut().previous = Some(new_node.clone());
                self.head = Some(new_node);
                println!("Added head node with hash value {}", hash_value);
            }else{
                let temp = self.lookup_node_mut(hash_value);
                new_node.as_ref().borrow_mut().next = Some(temp.clone());
                new_node.as_ref().borrow_mut().previous = Some(temp.borrow().previous.as_ref().unwrap().clone());
                println!("Added node with hash value {}", hash_value);
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_distance() {
        let ring = HashRing::new(5);
        assert_eq!(8, ring.distance(29, 5));
        assert_eq!(15, ring.distance(29, 12));
        assert_eq!(24, ring.distance(5, 29));
    }


}