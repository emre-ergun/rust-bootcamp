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

}


fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}