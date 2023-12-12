use std::fs::File;

fn main() {
    let file = File::open("example.txt")
                        .unwrap();
    let file = File::open("example.txt")
                        .expect("Failed to open the file!");

    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Failed to open file: {error:?}")
    //     }
    // };

    if let Err(msg) = get_user_id("emre") {
        println!("{msg}");
    } 
    // let v = vec!["one", "two", "three"];
    // println!("{}", v[3]);
    // panic!("Something went horribly wrong!");
}

fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("User name can not be empty".to_owned())
    } else {
        Ok(1)
    }
    
}