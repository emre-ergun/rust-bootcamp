fn main() {
    let mut s1 = String::from("Rust"); //heap allocated string
    let r1 = &s1;
    print_string(r1);
    let r2 = &mut s1;
    add_to_string(r2);
    println!("s1 is {s1}");

    let s2 = generate_string();
    print_string(&s2)
}

// we can not return a borrowed value from the function. Because it will be no longer available after function returns it.
// fn generate_string1() -> &String {
//     let s = String::from("Engram");
//     &s
// } // s is dropped after function returns

fn generate_string() -> String {
    String::from("Engram")
}

fn add_to_string(p1: &mut String){
    p1.push_str(" is awesome!"); // we don't need to dereference p1 as (*p1).push_str(" is awesome!")
}
fn print_string(p1: &String) {
    println!("{p1}");
}