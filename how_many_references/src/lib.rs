pub use std::rc::Rc;

pub struct Node {
    pub value: Vec<Rc<String>>,
}

impl Node {
    pub fn new(value: Vec<Rc<String>>) -> Node {
        Node { value }
    }
    pub fn add_ele(&mut self, v: Rc<String>) {
        self.value.push(v);
    }
    pub fn rm_all_ref(&mut self, v: Rc<String>) {
        self.value.retain(|value| !Rc::ptr_eq(value, &v))
    }
}

pub fn how_many_references(value: &Rc<String>) -> usize {
    Rc::strong_count(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn how_many_references_test() {
        let a = Rc::new(String::from("a"));
        let b = Rc::new(String::from("b"));
        let c = Rc::new(String::from("c"));

        let a1 = Rc::new(String::from("a"));

        let mut new_node = Node::new(vec![a.clone()]);
        new_node.add_ele(b.clone());
        new_node.add_ele(a.clone());
        new_node.add_ele(c.clone());
        new_node.add_ele(a.clone());

        assert_eq!(how_many_references(&a), 4);
        assert_eq!(how_many_references(&b), 2);
        assert_eq!(how_many_references(&c), 2);
        new_node.rm_all_ref(a1.clone());
        new_node.rm_all_ref(a.clone());

        assert_eq!(how_many_references(&a), 1);
        assert_eq!(how_many_references(&b), 2);
        assert_eq!(how_many_references(&c), 2);
    }
}
