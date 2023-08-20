fn main() {
    let s1 = String::from("Engram");
    print_string(s1);
    // one of the solution of the problem is to clone s1
    //print_string(s1.clone()); // used instead of moving ownership, the valuse is cloned.
    //println!("s1 is: {s1}"); // error because ownership of s1 is moved into the print_string function

    let s3 = generate_string();
    println!("s3 is: {s3}");

    let s4 = add_to_string(s3);
    println!("s4 is: {s4}");

    let x = 10;
    let _y = x;
    print_integer(x); // value of x is copied into function, not moved
    println!("x is: {x}");

}

fn print_integer(i: i32) {
    println!("i: {i}");
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}
fn generate_string() -> String {
    String::from("Rust")
}

fn print_string(p: String) {   
    println!("{}", p);
}//p is dropped.