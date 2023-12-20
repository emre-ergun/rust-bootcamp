use std::collections::HashMap;
use declerative_macros::{hello, map};

fn main() {
    hello!();
    let _scores1: HashMap<String, u32> = HashMap::new();

    let mut scores2 = HashMap::new();
    scores2.insert("red team".to_owned(), 3);
    scores2.insert("blue team".to_owned(), 5);
    scores2.insert("green team".to_owned(), 2);

    let _scores3 = map!(String, i32);

    let scores4 = map!(
        "read team".to_owned() => 3,
        "blue team".to_owned() => 5,
        "green team".to_owned() => 2
    );

    println!("{scores4:#?}");
}
