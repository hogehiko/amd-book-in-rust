use core::hash;
use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc, sync::Mutex};


fn main() {
   println!("Hello, world!");

   
}


struct Node<'a>{
    hash_value: u64,
    resources: HashMap<u64, u64>,
    next: Option<&'a mut Node<'a>>,
    previous: Option<&'a mut Node<'a>>,
}


impl <'a> Node<'a>{
    fn new(hash_value: u64) -> Self{
        Self { hash_value, resources: HashMap::new(), next: None, previous: None }
    }

    fn next(&'a mut self) -> &'a mut Node<'a>{
        self.next.as_mut().unwrap()
    }
}

struct HashRing<'a>{
    head: Option<&'a mut Node<'a>>,
    k: u32,
    min: u64,
    max: u64,
}

impl<'a> HashRing<'a>{
    fn new(k: u32) -> Self{
        Self { head: None, k, min: 0, max: 2u64.pow(k) - 1 }
    }

    fn head(&self) -> &Node<'a>{
        // in initial state, head is itself.
        self.head.as_ref().unwrap()
    }

    fn head_mut(&mut self) -> &mut Node<'a>{
        self.head.as_mut().unwrap()
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

    fn lookup_node_mut(&mut self, hash_value: u64) -> &mut Node<'a>{
        if self.is_in_legal_range(hash_value){
            let mut temp = self.head();
            let next = temp.next();

            while self.distance(temp.hash_value, hash_value) >
                self.distance(next.hash_value, hash_value){
                    // temp = temp.nesxt;``
                    temp = temp.next();
                    if temp.hash_value == hash_value{
                        return temp
                    }
            }
            return next;
        }
        panic!("Hash value out of range");
    }

    pub fn add_resource(&mut self, hash_value: u64){
        if self.is_in_legal_range(hash_value){
            println!("Adding a resource {} ...", hash_value);
            let target_node = self.lookup_node_mut(hash_value);
            target_node.borrow_mut().resources.insert(hash_value, hash_value);
            println!("Added resource with hash value {} to node {}", hash_value, target_node.borrow().hash_value);
        }
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
                new_node.as_ref().borrow_mut().previous.as_ref().unwrap().borrow_mut().next = Some(new_node.clone());
                new_node.as_ref().borrow_mut().next.as_ref().unwrap().borrow_mut().previous = Some(new_node.clone());
                println!("Added node with hash value {}", hash_value);
                println!("Its prev is {}", new_node.borrow().previous.as_ref().unwrap().borrow().hash_value);
                println!("Its next is {}", new_node.borrow().next.as_ref().unwrap().borrow().hash_value);
            }
        }
    }

    pub fn print_hash_ring(&self){
        println!("****");
        if self.head.is_none(){
            println!("Empty hash ring");
            return;
        }

        let mut temp = self.head();
        loop{
            {
                let node = temp.borrow();
                println!("Node hash value: {}", node.hash_value);
                println!("Resources: {:?}", node.resources.keys().collect::<Vec<&u64>>());
            }
            temp = temp.clone().borrow().next();
            if temp.borrow().hash_value == self.head().borrow().hash_value{
                break;
            }
        }
        println!("****")
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

    #[test]
    fn test_scenario(){
        // stdout used
        let mut hr = HashRing::new(5);
        hr.add_node(12);
        hr.add_node(18);
        hr.add_resource(24);
        hr.add_resource(21);
        hr.add_resource(16);
        hr.add_resource(23);
        hr.add_resource(2);
        hr.add_resource(29);
        hr.add_resource(28);
        hr.add_resource(7);
        hr.add_resource(10);
        hr.print_hash_ring();
    }

}