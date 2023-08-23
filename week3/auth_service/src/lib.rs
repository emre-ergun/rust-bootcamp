#![allow(unused_variables, dead_code)]
use rand::prelude::*;
use std::time::Duration;
use std::thread;

pub mod database;
mod auth_utils;

pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    let timeout = thread_rng().gen_range(1..5);
    thread::sleep(Duration::from_secs(timeout));
    println!("timeout: {timeout}");
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}

