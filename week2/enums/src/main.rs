struct Product {
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool,
}

enum ProductCategory {  
    Books,
    Clothing,
    Electrics,
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
        String::from("JSON string")
    }
}
fn main() { let category = ProductCategory::Electrics; let product = Product { name: String::from("TV"), category: category,
        price: 200.98,
        in_stock: true,
    };

    let cmd = Command::Undo;
    let cmd = Command::AddText(String::from("test"));
    let cmd = Command::MoveCursor(22, 0);
    let cmd = Command::Replace { 
        from: String::from("a"), 
        to: String::from("b"),
    };

    let json_string = cmd.serialize();
}
