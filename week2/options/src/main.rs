fn main() {
    let id: u32 = 1;
    let username = get_username(id);
    match username {
        Some(ref name) => println!("{name}"),
        None => println!("Error: No user in db which has id {id}"),
    }

    if let Some(name) = username {
        println!("{name}");
    }
}

fn get_username(user_id: u32) -> Option<String> {
    let db_result = String::from("Ferris");

    if user_id == 1 {
        Some(db_result)
    } else {
        None
    }
}
