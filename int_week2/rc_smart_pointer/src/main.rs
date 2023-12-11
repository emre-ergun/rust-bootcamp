use std::rc::Rc;
struct Database {

}

struct AuthService {
    db: Rc<Database>,
}

struct ContentService {
    db: Rc<Database>,
}

fn main() {
    let db = Rc::new(Database {});
    // Rc::clone() will not actually clone the variable, it simply
    // increments the reference counter

    // It is usefull with the single-threaded applications. for the multi-thread
    // applications we need to use atomic reference counting.
    let auth_service = AuthService { db: Rc::clone(&db)};
    let content_service = ContentService {db: Rc::clone(&db)};
}
