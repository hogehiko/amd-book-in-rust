use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};


type NodeRef = Rc<RefCell<Node>>;


trait NodeRefExt{
    fn insert(self, hash_value: u64, value: u64);

    fn hash_value(&self) -> u64;

    fn resources(&self) -> HashMap<u64, u64>;

    fn next(&self) -> NodeRef;

    fn previous(&self) -> NodeRef;

    fn set_next(&self, next: NodeRef);

    fn set_previous(&   self, previous: NodeRef);

    fn insert_resource(&self, hash_value: u64, value: u64);

    fn remove_resource(&self, hash_value: u64);

    fn set_finger(&self, index: u64, node: NodeRef);

    fn finger_table(&self) -> HashMap<u64, NodeRef>;

    fn inspect_finger_table(&self) -> Vec<(u64, u64)>;
}

impl NodeRefExt for NodeRef{
    fn insert(self, hash_value: u64, value: u64){
        self.as_ref().borrow_mut().resources.insert(hash_value, value);
    }

    fn hash_value(&self) -> u64 {
        self.as_ref().borrow().hash_value
    }

    fn resources(&self) -> HashMap<u64, u64>{
        self.as_ref().borrow().resources.clone()
    }

    fn next(&self) -> NodeRef{
        if let Some(next) = self.as_ref().borrow().next.as_ref(){
            next.clone()
        }else{
            panic!("Next node is None");
        }
    }

    fn previous(&self) -> NodeRef{
        if let Some(previous) = self.as_ref().borrow().previous.as_ref(){
            previous.clone()
        }else{
            panic!("Previous node is None");
        }
    }

    fn set_next(&self, next: NodeRef){
        self.as_ref().borrow_mut().next = Some(next);
    }

    fn set_previous(&self, previous: NodeRef){
        self.as_ref().borrow_mut().previous = Some(previous);
    }

    fn insert_resource(&self, hash_value: u64, value: u64){
        self.as_ref().borrow_mut().resources.insert(hash_value, value);
    }

    fn remove_resource(&self, hash_value: u64){
        self.as_ref().borrow_mut().resources.remove(&hash_value);
    }

    fn set_finger(&self, index: u64, node: NodeRef) {
        self.as_ref().borrow_mut().finger_table.insert(index, node);
    }

    fn finger_table(&self) -> HashMap<u64, NodeRef> {
        self.as_ref().borrow().finger_table.clone()
    }

    fn inspect_finger_table(&self) -> Vec<(u64, u64)> {
        self.as_ref().borrow().finger_table.iter().map(|(k, v)| (*k, v.hash_value())).collect()
    }
}

struct Node{
    hash_value: u64,
    resources: HashMap<u64, u64>,
    next: Option<NodeRef>, // if none, refer to itself
    finger_table: HashMap<u64, NodeRef>,
    previous: Option<NodeRef>, // if none, refer to itself
}


impl Node{
    fn new(hash_value: u64) -> Self{
        Self { hash_value, resources: HashMap::new(), next: None, previous: None, finger_table: HashMap::new() }
    }

    fn next(&self) -> NodeRef{
        if let Some(next) = self.next.as_ref(){
            next.clone()
        }else{
            panic!("Next node is None");
        }
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

    fn finger_ranges(&self) -> Vec<u64>{
        let mut ranges = vec![];
        for i in 0..self.k{
            ranges.push(2u64.pow(i));
        }
        ranges
    }

    fn head(&self) -> NodeRef{
        // in initial state, head is itself.
        self.head.as_ref().unwrap().clone()
    }

    fn head_mut(&mut self) -> &mut NodeRef{
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

    fn lookup_node_mut(&mut self, hash_value: u64) -> NodeRef{
        if self.is_in_legal_range(hash_value){
            let mut temp = self.head();
            // let next = temp.next();

            while self.distance(temp.hash_value(), hash_value) >
                self.distance(temp.next().hash_value(), hash_value){
                    // temp = temp.nesxt;``
                    temp = temp.next();
                    if temp.hash_value() == hash_value{
                        return temp
                    }
            }
            return temp.next();
        }
        panic!("Hash value out of range");
    }

    pub fn add_resource(&mut self, hash_value: u64){
        if self.is_in_legal_range(hash_value){
            println!("Adding a resource {} ...", hash_value);
            let target_node = self.lookup_node_mut(hash_value);
            let target_node_hash = target_node.hash_value();
            target_node.insert(hash_value, hash_value);
            println!("Added resource with hash value {} to node {}", hash_value, target_node_hash);
        }
    }


    fn move_resources(&mut self, dest: NodeRef, orig: NodeRef, delete_true: bool){
        let mut delete_list = vec![];
        for (i, j) in orig.resources().iter(){
            if self.distance(*i, dest.hash_value()) < self.distance(*i, orig.hash_value()) || delete_true{
                dest.insert_resource(*i, *j);
                delete_list.push(*i);
            }
        }

        for i in delete_list.iter(){
            orig.remove_resource(*i);
        }
    }

    fn add_node(&mut self, new_node: NodeRef){
        if self.is_in_legal_range(new_node.hash_value()){
            if self.head.is_none(){
                new_node.set_next(new_node.clone());
                new_node.set_previous(new_node.clone());
                self.head = Some(new_node.clone());
                println!("Added head node with hash value {}", new_node.hash_value());
            }else{
                let temp = self.lookup_node_mut(new_node.hash_value());
                new_node.set_next(temp.clone());
                new_node.set_previous(temp.previous());
                new_node.next().set_previous(new_node.clone());
                new_node.previous().set_next(new_node.clone());
                println!("Added node with hash value {}", new_node.hash_value());
                println!("Its prev is {}", new_node.previous().hash_value());
                println!("Its next is {}", new_node.next().hash_value());

                self.move_resources(new_node.clone(), new_node.next(),false);
                if new_node.hash_value() < self.head().hash_value(){
                    self.head = Some(new_node);
                }
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
                let node = temp.clone();
                println!("Node hash value: {}", node.hash_value());
                println!("Resources: {:?}", node.resources().keys().collect::<Vec<&u64>>());

                // print finger table
                println!("Finger table: {:?}", node.inspect_finger_table() )
            }


            temp = temp.next();
            if temp.hash_value() == self.head().hash_value(){
                break;
            }
        }
        println!("****")
    }

    fn build_finger_tables(&mut self){
        if self.head.is_none(){
            return;
        }

        let finger_ranges = self.finger_ranges();
        let mut temp = self.head();
        loop{
            for (i, range) in finger_ranges.iter().enumerate(){
                let finger_hash = (temp.hash_value() + range - 1) % (2u64.pow(self.k));
                let finger_node = self.lookup_node_mut(finger_hash);
                temp.set_finger(*range, finger_node);
            }

            temp = temp.next();
            if temp.hash_value() == self.head().hash_value(){
                break;
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

    #[test]
    fn test_scenario(){
        // stdout used
        let mut hr = HashRing::new(5);
        hr.add_node(RefCell::new(Node::new(12)).into());
        hr.add_node(RefCell::new(Node::new(18)).into());
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

        hr.add_node(RefCell::new(Node::new(5)).into());
        hr.add_node(RefCell::new(Node::new(27)).into());
        hr.add_node(RefCell::new(Node::new(30)).into());

        hr.build_finger_tables();
        hr.print_hash_ring();
    }

}