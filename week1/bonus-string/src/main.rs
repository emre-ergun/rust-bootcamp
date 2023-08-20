#[allow(unused_variables)]
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s1 = "Hello World! 🦀";
    let s2 = String::from("Hello World! 🦀");
    let s3 = "Hello World! 🦀".to_owned();
    let s4 = "Hello World! 🦀".to_string();
    let s5 = &s4[..];
    println!("{s5}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");
    s.replace_range(.., "baz");
    println!("{s}");
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    
    let s3 = s1 + &s2; // s1 is moved
    println!("{s3}");
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let s4 = format!("{s1}-{s2}-{s3}");
    println!("{s4}");

    let s1 = ["first", "second", "third", ].concat();
    let s2 = format!("{}{}", "first", "second");
    let s3 = concat!("first", "second");

    let s4 = String::from("test");
    let s5 = s4 + "okok"; // string type must be first

    let s1 = "🦀the🦀rust🦀developer🦀🦀🦀";
    let s2 = &s1[0..4];
    println!("{s2}");

    for byte in s1.bytes() {
        println!("{byte}");
    }

    for char in s1.chars() {
        println!("{char}");
    }

    for g in s1.graphemes(true) {
        println!("{g}");
    }

    let s1 = "Hello World";
    let s2 = String::from("Hello World");

    println!("{}", my_function(s1));
    println!("{}", my_function(&s2)); // deref coercion 
}

fn my_function(a: &str) -> String {
    format!("{a}")
}
