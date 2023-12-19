use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..20 {
            println!("{i} Spawned thread.");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..10 {
        println!("{i} Main thread");
    }

    handle.join().unwrap();
}
