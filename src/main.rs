use std::{rc::Rc, sync::Mutex};




fn main() {
   println!("Hello, world!");

   
}

struct Node{
    hash_value: u64,
    resources: Vec<u64>,
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

    fn lookup_node_mut(&mut self, hash_value: u64) -> Option<&mut Node>{
        if self.is_in_legal_range(hash_value){
            let Some(node) =  self.head.as_mut() else {
                return None;
            };
            while self.distance(node.hash_value, hash_value) > 
                self.distance(node.next.as_ref().unwrap().hash_value, hash_value){
                    let Some(next_node) = node.next.as_mut() else {
                        break;
                    };
                    node = next_node;
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