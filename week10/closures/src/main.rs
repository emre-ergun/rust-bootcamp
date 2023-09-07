struct Credentials<T> 
where T: Fn(&str, &str) -> bool
{
    username: String,
    password: String,
    validator: T,
}

impl<T> Credentials<T> 
where T: Fn(&str, &str) -> bool
{
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    let validator = |username: &str, password: &str| {
        !username.is_empty() && !password.is_empty()
    };

    let week_password = "123456password!".to_owned();
    let validator2 = |username: &str, password: &str| {
        !username.is_empty() &&
        !password.is_empty() &&
        password.len() > 8 &&
        password.contains(['!', '@', '#', '$', '%', '^', '&', '*']) &&
        password != week_password
    };

    let credential = Credentials {
        username: "Engram".to_owned(),
        password: "123456password!".to_owned(),
        validator: validator2,
    };


    println!("{}", validate_credentials(&credential.username, &credential.password));
    println!("{}", credential.is_valid());

    let password_validator = get_password_validator(5, true);
    let defauld_creds = get_default_credentials(password_validator);

}

fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}

fn get_default_credentials<T>(f: T) -> Credentials<T>
where T: Fn(&str, &str) -> bool
{
    Credentials { 
        username: "guest".to_owned(), 
        password: "password123!".to_owned(), 
        validator: f 
    }
}

fn get_password_validator(min_len: usize, special_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if special_char {
        Box::new(move |_: &str, password: &str| {
            !password.len() >= min_len &&
            password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
        })
    } else {
        Box::new(move |_: &str, password: &str| !password.len() >= min_len) 
    }
}