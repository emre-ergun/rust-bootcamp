#![allow(unused_variables, dead_code)]

pub fn login(creds: models::Credentials) {
    //authenticate
    crate::database::get_user();
}

fn logout() {
    //deauthenticate
}

pub mod models;