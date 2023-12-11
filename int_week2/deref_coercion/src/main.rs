use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T,
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> Self {
        MySmartPointer { value }
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
    let s = Box::new("Lets Get Rusty!".to_owned());
    // because of implicit deref coercion, we can pass Box<String>
    // type into the function as &str
    // in this case &Box -> &String -> &str
    print(&s);

    let s = MySmartPointer::new(
        Box::new("Lets Get Rusty".to_owned())
    );
    // &MySmartPointer -> &Box -> &String -> &str
    // &(***s);
    print(&s);
}

fn print(s: &str) {
    println!("{s}");
}
