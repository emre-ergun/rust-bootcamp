#![allow(dead_code)]
#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}
fn main() {
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0"
    ];

    let mut good_students = Vec::new();

    for s in students {
        let mut s = s.split(' ');
        let name = s.next();
        let gpa = s.next();

        if let (Some(name), Some(gpa)) = (name, gpa) {
            let name = name.to_owned();
            let gpa: f32= gpa.to_owned().parse().unwrap();

            if gpa >= 3.25 {
                good_students.push(Student {
                    name,
                    gpa
                });
            }
        }
    }

    println!("good students:");
    for student in good_students {
        println!("{student:?}");
    }

    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0"
    ];

    let bad_students: Vec<Student> = students
                    .iter()
                    .map(|s| {
                        let mut s = s.split(' ');
                        let name = s.next()?.to_owned();
                        let gpa: f32 = s.next()?.to_owned().parse().ok()?;

                        Some(Student {
                            name,
                            gpa
                        })
                    })
                    .flatten()
                    .filter(|s| s.gpa < 3.25)
                    .collect();

    println!("bad students:");
    for student in bad_students {
        println!("{student:?}");
    }

    let bad_students: Vec<Student> = students
                    .iter()
                    .filter_map(|s| {
                        let mut s = s.split(' ');
                        let name = s.next()?.to_owned();
                        let gpa: f32 = s.next()?.to_owned().parse().ok()?;

                        if gpa < 3.25 {
                            Some(Student {
                                name,
                                gpa
                            })
                        } else {
                            None
                        }
                    })
                    .collect();

    println!("bad students1:");
    for student in bad_students {
        println!("{student:?}");
    }
}
