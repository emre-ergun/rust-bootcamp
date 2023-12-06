fn main() {
    let tweet = String::from(
        "This is a tweet and it is very very long."
    );

    let trimmed_tweet = trim_tweet(&tweet); // string slice
    println!("{trimmed_tweet}");
    
    let tweet2 = "This is a tweet and it is very very long.";
    let trimmed_tweet2 = trim_tweet(tweet2);
    println!("{trimmed_tweet2}");

    let a = [1, 2, 3, 4, 5, 6, 7, 8];
    let a_slice = &a[..3];
    println!("{:?}", a_slice);

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let v_slice = &v[..5];
    println!("{:?}", v_slice);
}

fn trim_tweet(tweet: &str) -> &str {
    &tweet[..20]
}