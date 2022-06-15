#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: vec![],
            receipt: vec![],
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for product in &s.products {
            if ele == product.0 {
                self.items.push(product.clone());
                break;
            }
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut receipt_vec: Vec<f32> = self.items.iter().map(|x| x.1).collect();
        receipt_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let coef = receipt_vec[receipt_vec.len() / 3..].iter().sum::<f32>()
            / receipt_vec.iter().sum::<f32>();

        self.receipt = receipt_vec
            .iter()
            .map(|x| (x * 100.0 * coef).round() / 100.0)
            .collect();

        self.receipt.clone()
    }
}
