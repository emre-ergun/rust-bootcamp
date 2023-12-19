use std::thread;
use std::sync::mpsc; // Multi-producer Single-consumer

fn main() {

    let (tx, rx) = mpsc::channel();

    let sentences = [
        "!dlroW olleH".to_owned(),
        ".tsurT eW tsuR nI".to_owned(),
        "!ytsuR teG s'teL".to_owned(),
        "!tsuB ro tsuR".to_owned(),
    ];

    for sentence in sentences {
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let reversed: String = sentence.chars()
            .rev()
            .collect();

            tx_clone.send(reversed).unwrap();
        });
    }

    drop(tx); // without dropping main tx instance. it will wait forever.
              // because we use clones of tx instance.

    for sentence in rx{
        println!("{sentence}");
    }
}
