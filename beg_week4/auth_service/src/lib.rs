#![allow(dead_code, unused_variables)]

mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        Status::Connected
    }

    pub fn get_user() {
        // get user from database
    }
}

mod auth_utils {
    pub fn login(creds: models::Credential) {
        // authenticate
        crate::database::get_user();
    }
    
    fn logout() {
        // log user out...
    }

    pub mod models {
        pub struct Credential {
            username:String,
            password: String,
        }
    }
}

use auth_utils::models::Credential;
use database::Status;

pub fn authenticate(creds: Credential) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
} 