fn main() {
    if let Some(username) = get_usarname(1) {
        println!("Hello, {}!", username);
    };
}

fn get_usarname(user_id: u32) -> Option<String> {
    let query = format!("GET username FROM users WHERE id={user_id}");
    query_db(query).ok() // ok() method changes result enum to option enum
}


fn query_db(query: String) -> Result<String, String>{
    if query.is_empty() {
        Err(String::from("Query is empty"))
    } else {
        Ok(String::from("Engram"))
    }
}