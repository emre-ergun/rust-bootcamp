fn main() {
    let mut v: Vec<String> = Vec::new();
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));
    v.push(String::from("Four"));
    println!("{:?}", v);

    let v2 = vec![1, 2, 3, 4, 5];

    let s = &v[0]; // can panic because of bound of vector
    println!("{s}");
    let s = v.remove(0);
    println!("{:?}", v);

    if let Some(s) = v.get(0) {
        println!("{s}");
    }

    for s in &mut v {
        s.push_str("!");
    }

    for s in &v {
        println!("{s}");
    }

    let mut v3 = vec![];

    // for s in v {
    //     v3.push(s);
    // }

    // we can also use into_iter() method
    // self
    for s in v.into_iter() {
        v3.push(s);
    }

        println!("{:?}", v3);
    //reference self
    for s in v3.iter() {
        println!("{s}");
    }

    for s in v3.iter_mut() {
        *s = s.to_uppercase();
    }

    println!("{:?}", v3);
}
