fn main() {
    // if-else
    let x = 5;
    if x > 0 {
        println!("x is positive");
    } else {
        println!("x is negative");
    }

    let _b = if x > 5 {1} else {-1}; // both of branches should return same type

    // loop
    'outer: loop {
        println!("loop forever");
        loop {
            break 'outer; // break the loop 'outer. if the label is not specified, it would break the closest loop
        }
    }

    let _x = loop {
        break 5;
    };

    // while loop

    let mut a = 0;
    while a < 5 {
        println!("a is: {}", a);
        a += 1;        
    }

    // for loop
    let ar = [1, 2, 3, 4, 5];

    for element in ar {
        println!("{}", element);
    }
}
