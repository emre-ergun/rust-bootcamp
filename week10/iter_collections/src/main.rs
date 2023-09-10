use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("red team".to_owned(), 2);
    scores.insert("green team".to_owned(), 6);
    scores.insert("blue team".to_owned(), 8);

    let mut scores_iter = scores.iter_mut();
    let _first = scores_iter.next();

    for (team, score) in &scores {
        println!("{team} Got: {score} points!");
    }
}
