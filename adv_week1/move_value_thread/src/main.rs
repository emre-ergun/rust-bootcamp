use std::thread;
use std::time::Duration;

fn main() {
    let s = "Let's Get Rusty".to_owned();

    let handle = thread::spawn(move || {
        println!("{s}");
        thread::sleep(Duration::from_millis(1));
    });

    handle.join().unwrap();
}
