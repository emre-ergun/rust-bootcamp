struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand { name, payload }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print(&self) {
        println!("{}", self.payload);
    }
}
fn main() {
    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "https://engramsoft.com".to_owned()
    );
    cmd1.print();

    let cmd2 = BrowserCommand::new(
        "zoom".to_owned(),
        200,
    );
    //cmd2.print(); // error because payload2 is not string

    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();

    serialize_payload(p1);
    serialize_payload(p2);
}

fn serialize_payload<T>(payload: &T) -> String {
    "serialize".to_owned()
}
