use engrams_auth_service;

fn main() {

    let creds = engrams_auth_service::Credentials {
        username: String::from("Engram"),
        password: String::from("<PASSWORD>"),
    };
    
    engrams_auth_service::authenticate(creds)

}