fn main() {
    let mut s1 = String::from("Rust");
    let r1 = &s1;
    print_string(r1);
    println!("print outside of the function\t: {s1}");
    let r2 = &mut s1;
    add_to_string(r2);
    print_string(&s1);

    let mut s2 = generate_string();
    print_string(&s2);
    add_to_string(&mut s2);
    println!("print outside of the function\t: {s2}");
}

fn generate_string() -> String {
    String::from("Ferris")
}
fn add_to_string(p1: &mut String){
    p1.push_str(" is amazing!");
}
fn print_string(p1: &String) {
    println!("print inside of the function\t: {p1}");
}
