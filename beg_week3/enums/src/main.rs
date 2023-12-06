#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    in_stock: bool,
    category: ProductCategory,
}

#[derive(Debug)]
enum ProductCategory {
    Books,
    Clothing,
    Electronics,
}

enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    },
}

impl Command {
    fn serialize(&self) -> String {
        String::from("JSON string...")
    }
}
fn main() {
    let category = ProductCategory::Electronics;
    let product = Product {
        name: String::from("TV"),
        price: 130.85,
        category,
        in_stock: true,
    };
    println!("{:?}", product);

    let cmd = Command::Undo;
    let cmd = Command::AddText(String::from("Hello"));
    let cmd = Command::MoveCursor(23, 105);
    let cmd = Command::Replace { 
        from: String::from("from"), 
        to: String::from("to") 
    };

    let json_string = cmd.serialize();
    println!("{json_string}");
}
