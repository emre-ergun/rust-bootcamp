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
    let query = format!(
        "GET username FROM users WHERE id={id}"
    );

    let db_result = query_db(query);
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query string is empty!"))
    } else {
        Ok(String::from("Ferris"))
    }
}