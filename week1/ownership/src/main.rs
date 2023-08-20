#[allow(unused_variables)]
fn main() {
    {
        let s2 = String::from("Rust");
    }
    //println!("{s2}"); // error because s2 is dropped when it is goes out of the block.
    
    let s1 = String::from("Rust"); // heap allocated string
    println!("s1: {s1}");

    let s2 = s1; // implicit move semantic
    //println!("{s1}"); // error because s1 is borrowed at previos line.
                        // it means that s1 value moved into s2
    // but we can print s2 now
    println!("s2: {s2}");


    // what happens if we clone s1 to s2
    let s1 = String::from("Engram");
    let s2 = s1.clone(); // clone allocates new memory in the heap and assigns pointer's value to variable s2
    println!("s1: {s1}");
    println!("s2: {s2}");

    // for some primitve types the clone method is called autmatically
    // for integers, floats, booleans, or characters. They are cheap to clone
    let x = 15;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");

} //s1 will be dropped
