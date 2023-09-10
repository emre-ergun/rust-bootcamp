struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}

struct PersonIterator {
    values: Vec<String>,
}

impl Iterator for PersonIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None;
        } 
        Some(self.values.remove(0))
    }
}

// impl IntoIterator for Person {
//     type Item = String;
//     type IntoIter = PersonIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         PersonIterator {
//             values: vec! [
//                 self.first_name,
//                 self.last_name,
//                 self.occupation,
//             ]
//         }
//     }
// }

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec! [
            self.first_name,
            self.last_name,
            self.occupation,
        ].into_iter()
    }
}

fn main() {
    let person = Person {
        first_name: "Engram".to_owned(),
        last_name: "Soft".to_owned(),
        occupation: "Software Engineer".to_owned(),
    };

    let mut iter = person.into_iter();

    while let Some(value) = iter.next() {
        println!("{value}");
    }

    let person = Person {
        first_name: "Engram".to_owned(),
        last_name: "Soft".to_owned(),
        occupation: "Software Engineer".to_owned(),
    };
    for items in person {
        println!("{items}");
    }
}
