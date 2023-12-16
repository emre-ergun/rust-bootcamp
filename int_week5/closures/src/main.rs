struct Credentials<T> where T: Fn(&str, &str) -> bool {
    username: String,
    password: String,
    validator: T,
}

impl<T> Credentials<T> where T: Fn(&str, &str)->bool{
    fn is_valid(&self) -> bool {
        // we need to wrap closure with paranthesis
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    // rust is able to infer return type
    let validator = |username: &str, password: &str| {
        !username.is_empty() && !password.is_empty()
    };

    let week_password = "password123!".to_owned();
    // Fn - immutable borrow variables in environment
    // FnMut - mutable borrow variables in environment.
    //          can change environment
    // FnOnce - Take ownership of variables in the environment. Can only be called once.
    //          move keyword can use to take ownership of the variable in the environment
    let validator2 = |username: &str, password: &str| {
        !username.is_empty() &&
        !password.is_empty() &&
        password.len() > 8 &&
        password.contains(['!', '@', '#', '$', '%', '^', '&', '*']) &&
        password != week_password // immutable borrowed !!!!
    };

    println!("{week_password}");

    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password123!*".to_owned(),
        validator: validator2
    };

    

    println!("{}", validate_credentials(&creds.username, &creds.password));
    println!("{}", validator(&creds.username, &creds.password));
    println!("{}", creds.is_valid());

    let password_validator = get_password_validator1(8, true);
    let default_creds = get_default_creds(password_validator);
    println!("default: {}", default_creds.is_valid());
}

fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}

fn get_default_creds<T>(f: T) -> Credentials<T> 
where T: Fn(&str, &str) -> bool {
    Credentials {
        username:"guest".to_owned(),
        password:"password123!".to_owned(),
        validator: f,
    }
}

fn get_password_validator(min_len: usize) -> impl Fn(&str, &str)->bool {
    move |_: &str, password: &str| !password.len() >= min_len
}

fn get_password_validator1(min_len: usize, special_char: bool) -> Box<dyn Fn(&str, &str)->bool> {
    if special_char {
        Box::new(move |_: &str, password: &str| {
            !password.len() >= min_len &&
            password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
        })
    } else {
        Box::new(move |_: &str, password: &str| !password.len() >= min_len)
    }
}