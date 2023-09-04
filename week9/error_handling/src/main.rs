use std::{collections::HashMap, io, num::ParseIntError};

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32,
}

#[derive(Debug)]
struct Card { 
    number: u32,
    exp: Expiration,
    cvv: u32
}

fn main() {
    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27 123"),
        ("Bob", "1234567 12 27 123"),
    ]);

    println!("Enter name: ");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("\nCredit card info: {card:#?}"),
        Err(e) => println!("Error: {e}"),
    }

}

fn get_credit_card_info(credit_cards: &HashMap<&str, &str>, name: &str) -> Result<Card, String> {
    let card_string = credit_cards.get(name)
                                        .ok_or(format!("No credit card found for {name}"))?;
    let card = parse_card(card_string)?;

    Ok(card)
}

fn parse_card(card: &str) -> Result<Card, String> {
    let mut numbers = parse_card_numbers(card)
                                    .map_err(|e| e.to_string())?;

    let len  = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(format!(
            "Incorrect number of elements parsed. Expected {expected_len} but get {len}. Elements {numbers:?}"
        ))
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration {year, month},
        cvv
    })
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParseIntError> {
    let numbers = card
                        .split(" ")
                        .into_iter()
                        .map(|s| s.parse())
                        .collect::<Result<Vec<u32>, _>>()?;

    Ok(numbers)
}