enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    },
}

impl Command {
    fn serialize(&self) -> String {
        let json_string = match self {
            Command::Undo => String::from(
                "{ \"cmd\": \"undo\" }"
            ),
            Command::Redo => String::from(
                "{ \"cmd\": \"redo\" }"
            ),
            Command::AddText(s) => {
                format!(
                    "{{\
                        \"cmd\": \"add_text\", \
                        \"text\": \"{s}\" \
                    }}"
                )
            },
            Command::MoveCursor(line, column) => {
                format!(
                    "{{\
                        \"cmd\": \"move_cursor\", \
                        \"line\": {line}, \
                        \"column\": {column} \
                    }}"
                )
            },
            Command::Replace { from, to } => {
                format!(
                    "{{\
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\" \
                    }}"
                )
            }
        };

        json_string
    }
}

fn main() {
    let cmd = Command::MoveCursor(12, 0);
    let json_string = cmd.serialize();
    println!("{json_string}");
    let cmd = Command::Replace { 
        from: String::from("Rust"), 
        to: String::from("Ferris") 
    };
    let json_string = cmd.serialize();
    println!("{json_string}");
}
