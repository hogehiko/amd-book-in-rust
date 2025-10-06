use core::hash;
use std::{collections::HashMap, rc::Rc, sync::Mutex};


fn main() {
   println!("Hello, world!");

   
}

struct Node{
    hash_value: u64,
    resources: HashMap<u64, u64>,
    next: Option<Rc<Node>>,
    previous: Option<Rc<Node>>,
}

impl Node{
    fn new(hash_value: u64) -> Self{
        Self { hash_value, resources: vec![], next: None, previous: None }
    }

}

struct HashRing{
    head: Option<Rc<Node>>,
    k: u32,
    min: u64,
    max: u64,
}

impl HashRing{
    fn new(k: u32) -> Self{
        Self { head: None, k, min: 0, max: 2u64.pow(k) - 1 }
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

    fn lookup_node_mut(&mut self, hash_value: u64) -> &mut Node{
        if self.is_in_legal_range(hash_value){
            let Some(node) =  self.head.as_mut() else {
                panic!("No nodes in the ring");
            };
            let mut temp = node;
            while self.distance(temp.hash_value, hash_value) > 
                self.distance(temp.next.as_ref().unwrap().hash_value, hash_value){
                    temp = temp.next;
                let Some(next_node) = temp.next.as_mut() else {
                    break;
                };
            }
        }
    }

    fn move_resources(self, dist: &Node, orig: &Node, delete_true: bool){
        let mut delete_list = vec![];
        for (i, j) in &dist.resources{
            if self.distance(i , dist.hash_value < self.distance(i, orig.hash_value) || delete_true){
                dist.resources[*i] = *j;
                delete_list.push(*i);
                println!("move resource {} from {} to {}", i, orig.hash_value, dist.hash_value);
            }
        }

        for i in delete_list{
            orig.resources.remove(&i);
        }
    }

    fn add_node(&mut self, hash_value: u64) -> bool{
        if self.legal_range(hash_value){
            let mut new_node = Rc::new(Node::new(hash_value));

            if self.head.is_none(){
                new_node.next = Some(new_node.clone());
                new_node.previous = Some(new_node.clone());
                self.head = Some(new_node);
                println!("Adding a head node {}", hash_value);
            }else{
                let temp = self.lookup_node_mut(hash_value).unwrap();
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