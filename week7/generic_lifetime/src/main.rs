use rand;

fn main() {
    // example 1
    let player1 = String::from("player 1");
    let player2 = String::from("player 2");
    let result = first_turn(&player1[..], &player2[..]);
    println!("Player going first is {result}");

    // example 2
    // example is still valid because we are printing result value
    // at the same scope with the scope of the shortest value.
    let player1 = String::from("player 1");
    {
        let player2 = String::from("player 2");
        let result = first_turn(&player1[..], &player2[..]);
        println!("Player going first is {result}");
    }

    // example 3
    // error because we are printing result value after the value which has shortest lifetime moved out of scope.
    // let result1;
    // let player1 = String::from("player 1");
    // {
    //     let player2 = String::from("player 2");
    //     result1 = first_turn(&player1[..], &player2[..]);
    // }
    // println!("Player going first is {result1}");
    
    // example 4
    // there is no error anymore. Because lifetime specifier we use specify that 
    // lifetime of the return value is related to p1 argument so that the return value and
    // first argument live long enough.
    let result1;
    let player1 = String::from("player 1");
    {
        let player2 = String::from("player 2");
        result1 = first_turn1(&player1[..], &player2[..]);
    }
    println!("Player going first is {result1}");


    // example 5
    let result = first_turn2("hello", "Engram");
    println!("{result}");
}


fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    let rand_number = rand::random();
    println!("True or False ? {rand_number}");

    if rand_number {
        p1
    } else {
        p2
    }
}

fn first_turn1<'a>(p1: &'a str, p2: &str) -> &'a str {
    p1
}

fn first_turn2(_p1: &str, _p2: &str) -> &'static str {
    let s1: &'static str = "Lets Get Rusty";
    s1
}