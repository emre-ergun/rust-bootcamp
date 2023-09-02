fn main() {
    // LIFETIME of OWNED VALUES
    // s1 is variable that is heap allocated
    let s1 = String::from("Let's Get Rusty");
    println!("{s1}");
    
    {
        //life of s2 starts at line 8 and ends at line 9 when it goes out of the scope.
        let s2 = String::from("Let's Get Rusty");
    }
    //println!("{s2}"); // error because s2 went out of scope
    let s2 = s1;
    //println!("{s1}"); // error because s1 moved to s2 and s1 is no longer valid

    // LIFETIME of REFERENCED VALUES
    let r1;
    {
        let s1 = String::from("Let's Get Rusty");
        r1 = &s1;
        println!("{r1}");
    }
    //println!("{r1}"); // to solve error println should be moved into inner scope

    // normally rust borrowing rules don't allow to have mutable and unmutable at the same time
    // rust is smart as much as enough to realize r1 reference is not used after it is reference as mutable r2
    // r1 is declared at line 28 and used at line 29, r2 is declared at line 30 and last used at line 32
    let mut s1 = String::from("Lets Get Rusty");
    let r1 = &s1;
    println!("r1: {r1}");
    let r2 = &mut s1;
    r2.push_str("!");
    println!("r2: {r2}");


}
