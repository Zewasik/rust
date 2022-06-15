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
        let mut temp: Vec<f32> = self.items.iter().map(|x| x.1).collect();
        temp.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let num_discount = temp.len() / 3;
        let new_sum: f32 = temp[num_discount..].iter().sum();
        let sum: f32 = temp.iter().sum();
        let coef = (new_sum * 100.0).round() / sum / 100.0;
        temp = temp
            .iter()
            .map(|x| (x * 100.0 * coef).round() / 100.0)
            .collect();

        println!("sum: {}, new sum: {}, coef: {}", sum, new_sum, coef);
        self.receipt = temp.clone();
        temp
    }
}
