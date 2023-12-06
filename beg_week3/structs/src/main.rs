struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product {
    fn new(name: String, price: f32) -> Product {
        Product { name: name, price: price, in_stock: true }
    }
    // associated function
    fn get_default_sales_tax() -> f32 {
        0.1
    }

    fn calculate_sales_tax(&self) -> f32 {
        self.price * Product::get_default_sales_tax()
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} is bought!");
        let bill_number = 123;
        bill_number
    }
}

fn main() {
    let mut book = Product::new(String::from("Book"), 28.85);

    println!("Default sales tax: {}", Product::get_default_sales_tax());
    let sale_tax = book.calculate_sales_tax();
    println!("sales tax: {sale_tax}");
    
    book.set_price(32.25);
    let sale_tax = book.calculate_sales_tax();
    println!("sales tax: {sale_tax}");
    book.buy();
}

