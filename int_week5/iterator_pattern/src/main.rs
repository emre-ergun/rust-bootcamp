
// trait Iterator1<Item> {
//     fn next(&mut self)-> Option<Item>;
// }

// struct MyStruct {}

// impl Iterator1<String> for MyStruct {
//     fn next(&mut self)-> Option<String> {
//         None
//     }
// }

// impl Iterator1<i32> for MyStruct {
//     fn next(&mut self)-> Option<i32> {
//         None
//     }
// }

// trait Iterator {
//     type Item;
    
//     fn next(&mut self) -> Option<Self::Item>;
// }

// impl Iterator for MyStruct {
//     type Item = String;
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

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

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.first_name,
            self.last_name,
            self.occupation
        ].into_iter()
    }
}
fn main() {
    // let mut m = MyStruct {};
    // // let item: Option<String> = m.next(); // explicitly annonate the type as Option<String>.
    // let item = m.next();

    let p = Person {
        first_name: "Emre".to_owned(),
        last_name: "Ergün".to_owned(),
        occupation: "Software Developer".to_owned(),
    };

    // let mut i = p.into_iter();

    // println!("{:?}", i.next());
    // println!("{:?}", i.next());
    // println!("{:?}", i.next());
    // println!("{:?}", i.next());

    for item in p {
        println!("{:?}", item);
    }
}
