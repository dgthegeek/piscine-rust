pub type Item = (String, f32);
#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<Item>,
}

impl Store {
    pub fn new(products: Vec<Item>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<Item>,
    pub receipt: Vec<f32>,
}

impl Default for Cart {
    fn default() -> Self {
        Self::new()
    }
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, name: String) {
        for product in s.products.iter() {
            if product.0 == name {
                self.items.push(product.clone());
            }
        }
    }
    
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.receipt = self.items.iter().map(|item| item.1).collect();
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let free = self.receipt.len() / 3;
        let total: f32 = self.receipt.iter().sum();
        let discount = self.receipt.iter().take(free).sum::<f32>();
        let total_discount = total - discount;
        let discount_per_item = (total_discount * 100.0 / total) / 100.0;
        self.receipt.iter_mut().for_each(|price| {
            *price = ((*price * discount_per_item * 100.0).round()) / 100.0;
        });
        self.receipt.clone()
    }
}
