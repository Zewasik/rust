#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        self.head = Some(Node {
            value,
            next: match self.head.take() {
                Some(value) => Some(Box::new(value)),
                None => None,
            },
        })
    }

    pub fn pop(&mut self) {
        self.head = match self.head.take() {
            Some(value) => match value.next {
                Some(s) => Some(*s),
                None => None,
            },
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        let mut res: usize = 0;
        let mut new_head = self.head.as_ref();

        loop {
            match new_head {
                Some(node) => {
                    res += 1;
                    new_head = if !node.next.is_none() {
                        Some(&node.next.as_ref().unwrap())
                    } else {
                        break res;
                    }
                }
                None => break res,
            }
        }
    }
}
