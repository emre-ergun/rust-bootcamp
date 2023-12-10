use rand;

fn main() {
    let player1 = String::from("player1");
    let player2 = String::from("player2");

    let result = first_turn(&player1, &player2);
    println!("Result: {result}");
}

fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random(){
        p1
    }  else {
        p2
    }
}

fn first_turn1(p1: &str, p2: &str) -> &'static str {
    let s = "Lets get rusty"; // string literals live during the program runs.
    s
}
