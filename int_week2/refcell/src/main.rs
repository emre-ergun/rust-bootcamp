use std::rc::Rc;
use std::cell::RefCell;

struct Database {
    max_connections: u32,
}

struct AuthService {
    db: Rc<RefCell<Database>>,
}

struct ContentService {
    db: Rc<RefCell<Database>>,
}

fn main() {
    let db = Rc::new(RefCell::new(Database {
        max_connections: 100,
    }));
    // Rc::clone() will not actually clone the variable, it simply
    // increments the reference counter

    // It is usefull with the single-threaded applications. for the multi-thread
    // applications we need to use atomic reference counting.
    let auth_service = AuthService { db: Rc::clone(&db)};
    let content_service = ContentService {db: Rc::clone(&db)};

    db.borrow_mut().max_connections = 200;
}
