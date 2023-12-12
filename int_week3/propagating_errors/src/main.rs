use std::{io::{self, Read}, fs::File};

fn main() {
    let contents = read_file("example.txt").unwrap();
    println!("{contents}");
    let contents = read_file2("example.txt").unwrap();
    println!("{contents}");

    let user = User {
        firstname: "Emre".to_owned(),
        lastname: "Ergun".to_owned(),
    };

    if let Some(initials) = get_initials(&user) {
        println!("Initials: {initials}");
    }
}

fn read_file(filename: &str) -> Result<String, io::Error>{
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_file2(filename: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}

struct User {
    firstname: String,
    lastname: String,
}

fn get_initials(user: &User) -> Option<String> {
    let first_initial = user.firstname.chars().next()?;
    let last_initial = user.lastname.chars().next()?;
    Some(format!("{first_initial}.{last_initial}"))
}