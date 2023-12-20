use procedural_macros::*;

trait Log {
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
}

#[derive(Debug, Log)]
struct Database {
    url: String,
    connections: u32,
}

impl Database {
    fn new(url: String) -> Database {
        Database {url, connections: 99}
    }

    fn connect(&mut self) {
        self.info(format!("new connection to {}", self.url).as_str());
        self.connections += 1;
        if self.connections >= 100 {
            // a lot of connections!
            self.warn(format!("100 or more connections open!").as_str());
        }
    }
}

fn main() {
    let mut db = Database::new("localhost:5433".to_owned());
    db.connect();
    //log_info!([TIME] starting program...);
}