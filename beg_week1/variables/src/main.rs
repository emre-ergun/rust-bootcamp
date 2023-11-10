fn main() {
    println!("Variables");

    // creation
    
    let a: i16 = 5;
    println!("a:{a}");

    // mutability
    
    let mut b = 10;
    println!("b:{b}");
    b = 5;
    println!("b:{b}");
    
    // shadowing 
    
    let c = 10;
    println!("c:{c}");
    let c = 25; // second variable will shadow first variable in same scope.
    println!("c:{c}");
    
    // scope
    
    let d = 30;
    {
        let d = 40;
        println!("d:{d}");
    }
    println!("d:{d}");
}
