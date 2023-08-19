fn main() {
    let z = my_function(6);
    println!("my_function returned the value: {}", z);
}

fn my_function(x: i32) -> i32 {
    println!("my_function called with: {}", x);
    let y = 10;
    y // it is only valid for lates expression. otherwise we should use "return" keyword.
}