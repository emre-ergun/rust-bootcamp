fn main() {
    let s = "Hello Rust Developers"; // in rust, all string literatls are string slices.
    println!("{s}");

    let tweet = String::from(
        "This is my tweet and it is very very long"
    );

    let trimmed_tweet = &tweet[..20];
    println!("{trimmed_tweet}"); // string slice

    let trimmed_tweet = trim_tweet(&tweet);
    println!("{trimmed_tweet}");
    
    // there is no complaning about type mismatching between &String and &str because there is dref coercion.
    // rust automatically coerce &String to &str.
    let tweet2 = "This is my tweet and it is very very long";
    let trimmed_tweet = trim_tweet(tweet2);
    println!("{trimmed_tweet}");

    let ar = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let ar_slice = &ar[..5];

    println!("{:?}", ar_slice);
}   

fn trim_tweet(s: &str) -> &str {
    &s[..30]
}
