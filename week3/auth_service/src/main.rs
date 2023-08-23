use auth_service;

fn main() {

    let creds = auth_service::Credentials {
        username: String::from("Emre"),
        password: String::from("<PASSWORD>"),
    };
    
    auth_service::authenticate(creds)

}