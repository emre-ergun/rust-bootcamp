use std::{io, fs, error, num::ParseIntError};

fn main() {
    let i = parse_file("example.txt");

    match i {
        Ok(i) => println!("{i}"),
        Err(e) => {
            match e {
                ParseFileError::File => println!("There is error opening file!"),
                ParseFileError::Parse(e) => println!("Error: {e}"),
            }
        }
    }
}

enum ParseFileError {
    File,
    Parse(ParseIntError),

}

fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename)
                            .map_err(|e| ParseFileError::File)?;
    let i = s.parse()
                            .map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}

fn parse_file1(filename: &str) -> Result<i32, Box<dyn error::Error>> {
    let s = fs::read_to_string(filename)?;
    let i = s.parse()?;
    Ok(i)
}
