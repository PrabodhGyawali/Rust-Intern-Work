pub struct MenuItem {
    pub name: String,
    pub price: f32,
}

impl MenuItem {
    pub fn new(s: &str, p: f32) -> MenuItem {
        MenuItem {
            name: s.to_string(),
            price: p,
        }
    }
    pub fn change_price(&mut self, new_price: f32) {
        self.price = new_price;
    }
    pub fn as_string(&self) -> String {
        format!("{} - ${}", self.name, self.price)
    }
    pub fn change_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
    pub fn get_price(&self) -> f32 {
        self.price
    } // getter
}