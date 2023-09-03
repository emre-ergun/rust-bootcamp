use std::fs::File;

fn main() {
    let user_id = get_user_id("Engram").unwrap();
    println!("{user_id}");

    let file = File::open("example.txt").unwrap();

    // instead of match expressions below, we can use unwrap as above.
    // let file = match file {
    //     Ok(file) => file,
    //     Err(err) => {
    //         panic!("Failed to open file: {}", err);
    //     },
    // };
}

fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("User name can not be empy!".to_owned())
    } else {
        Ok(1495)
    }
}
