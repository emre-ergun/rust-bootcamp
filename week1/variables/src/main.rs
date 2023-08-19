fn main() {
    // creation
    let a = 23;     // implicit type annotation
                        // default annotation type is i32
    let a: u16 = 23;    // explicit type annotation
    println!("a = {}", a);

    // mutability
    let mut b = 25;
    println!("b = {}", b);
    b = 50;
    println!("b = {}", b);

    // shadowing
    let c = 10;
    let c = 50; // second variable shadowing first one
    println!("c = {}", c);

    // scope
    let d = 40;
    {
        let d = 50;
        println!("d(inner_scope) = {}", d);
        let e = 60;
        println!("e(inner_scope) = {}", e);
    }
    //println!("e(outer_scope) = {}", e); // it is defined in inner_scope
    println!("d(outer_scope) = {}", d);
}
