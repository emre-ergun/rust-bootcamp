fn main() {
    // panic by program
    let v = vec![
        "one",
        "two",
        "three",
    ];
    println!("{}", v[3]);
    // panic called by programmer
    panic!("Something went horribly wrong!");
}
