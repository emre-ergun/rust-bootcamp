fn main() {
    let username = get_username(2);
    match username {
        Some(name) => println!("{name}"),
        None => println!("No user found!"),
    }
    // or
    if let Some(username) = get_username(1) {
        println!("{username}");
    }

}
fn get_username(id: i32) -> Option<String> {
    let db_result = String::from("Ferris");
    if id == 1 {
        Some(db_result)
    } else {
        None
    }
}