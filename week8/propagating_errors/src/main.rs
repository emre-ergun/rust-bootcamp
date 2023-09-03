use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let contents = read_file("example.txt");

    let user = User {
        name: "Emre".to_owned(),
        surname: "Ergun".to_owned(),
    };

    println!("{}", get_initials(user).unwrap());
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}

struct User {
    name: String,
    surname: String,
}

fn get_initials(user: User) -> Option<String> {
    let first_initial = user.name.chars().next()?;
    let last_initial = user.surname.chars().next()?;

    Some(format!("{first_initial}.{last_initial}"))
}
