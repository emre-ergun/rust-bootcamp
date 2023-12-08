pub mod models;

pub fn login(creds: models::Credential) {
    // authenticate
    crate::database::get_user();
}

fn logout() {
    // log user out...
}
