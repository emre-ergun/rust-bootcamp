use std::io;
use std::fs;
use std::error;
use std::num::ParseIntError;

fn main() {
    let i = parse_file1("example.txt");

    match i {  
        Ok(i) => println!("Number is {i}"),
        Err(err) => {
            match err {
                ParseFileError::File => {
                    // ...
                },
                ParseFileError::Parse(err) => {
                    // ...
                }
            }
        }
    }
}

fn parse_file(filename: &str) -> Result<String, Box<dyn error::Error>> {
    let s = fs::read_to_string(filename)?;
    let i = s.parse()?;
    Ok(i)
}

enum ParseFileError {
    File,
    Parse(ParseIntError),
}

fn parse_file1(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename)
                .map_err(|err| ParseFileError::File)?;
    let i = s.parse()
                .map_err(|err| ParseFileError::Parse(err))?;
    Ok(i)
}


