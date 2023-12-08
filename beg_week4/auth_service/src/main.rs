use auth_service::{self, Credential, authenticate};

fn main() {
    let someone = Credential {
        username: String::from("Ferris"),
        password: String::from("123456"),
    };

    authenticate(someone);
}