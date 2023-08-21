struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product {
    fn new(name: String, price: f32) -> Product {
        Product {
            name,
            price,
            in_stock: true,
        }
    }
    fn calculate_sales_tax(&self) -> f32 {
        self.price * 0.1
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> i32 {
        let name = self.name;
        println!("The book {name} is bought");
        123
    }
}

fn main() {
    let mut book = Product::new(
        String::from("Book"),
        28.85,
    );
    book.in_stock = false;

    let sales_tax = book.calculate_sales_tax();
    println!("sales tax: {}", sales_tax);
    
    book.set_price(23.25);
    println!("sales tax: {}", book.calculate_sales_tax());

    book.buy();
}


