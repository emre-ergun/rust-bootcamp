use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T,
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> Self {
        MySmartPointer {
            value
        }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
fn main() {
    let s = MySmartPointer::new(
        Box::new(
            "Lets Get Rusty".to_owned()
        )
    );
    // this works because of a feature of Rust named as implicit deref coercion
    // in this case &MySmartPointer -> &Box -> &String -> &str
    // let s = &(***s);
    print(&s);

    // &MySmartPointer -> &String -> &str
    let s = MySmartPointer::new(
        "Hello EngramSoft".to_owned(),
    );
    print(&s);
}

fn print(s: &str) {
    println!("{s}");
}
