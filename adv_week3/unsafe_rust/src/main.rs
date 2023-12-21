fn main() {
    unsafe {
        // 1. Dereference a raw pointer
        // 2. Call an unsafe function
        // 3. Implement an unsafe trait
        // 4. Access/Modify a mutable static variable
        // 5. Access fields of a union.
    }

    // 1. Dereference a raw pointer
    let mut s = "Let's Get Rusty".to_owned();
    let r1 = &s as *const String;
    let r2 = &mut s as *mut String;

    let address = 0x012345usize;
    let r3 = address as *const String;

    unsafe {
        (*r2).push_str("!!!");
        println!("r1 is {}", *r1);
    }
}
