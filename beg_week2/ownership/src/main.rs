fn main() {
    let s1 = String::from("Rust");
    println!("s1 is: {s1}");
    
    let s2 = s1; // s1 moved to s2 implicitly
    let s3 = s2.clone();
    println!("s2 is: {s2}"); // s2 cloned and s3 is owner of the cloned value (different one)
    println!("s3 is: {s3}");
    
    let s1 = String::from("Rust");
    print_string(s1.clone());
    let s2 = generate_string();
    println!("s1 is: {s1}");
    println!("s2 is: {s2}");

    let n1 = generate_number(10);
    println!("number: {n1}");

    let s3 = add_to_string(s2);
    println!("s3: {s3}");
}

fn add_to_string(mut s1: String) -> String {
    s1.push_str(" is awesome!");
    s1
}

fn print_string(p1: String) {
    println!("Print in the function: {p1}");
}// p1 is dropped

fn generate_string() -> String {
    String::from("Ferris")
}

fn generate_number(n1: i32) -> i32 {
    let result = 25 + n1;
    result
}