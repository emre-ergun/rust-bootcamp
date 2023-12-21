//inline assembly
use std::arch::asm;

fn add(x: u64, y: u64) -> u64 {
    let result: u64;

    unsafe {
        // x86/x86-64 assembly
        asm!(
            "add {0}, {1}", 
            inout(reg) x => result, 
            in(reg) y
        );
    }

    result
}
// 3. Implement an unsafe trait
unsafe trait MyTrait {
    fn some_function(&self);
}

unsafe impl MyTrait for String {
    fn some_function(&self) {
        //..
    }
}

// 4. Access/Modify a mutable static variable
static mut COUNTER: u32 = 0;

fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}

fn main() {
    //unsafe {
        // 1. Dereference a raw pointer
        // 2. Call an unsafe function
        // 3. Implement an unsafe trait
        // 4. Access/Modify a mutable static variable
        // 5. Access fields of a union.
    //}

    // 1. Dereference a raw pointer
    let mut s = "Let's Get Rusty".to_owned();
    let r1 = &s as *const String;
    let r2 = &mut s as *mut String;

    let address = 0x012345usize;
    let _r3 = address as *const String;

    unsafe {
        (*r2).push_str("!!!");
        println!("r1 is {}", *r1);
    }

    // 2. Call an unsafe function
    unsafe {
        my_function();
    }

    // 3. Implement an unsafe trait
    let s = "some string".to_owned();
    s.some_function();

    // 4. Access/Modify a mutable static variable
    for _ in 0..10 {
        increment_counter();
    }
    unsafe {
        println!("counter:{COUNTER}");
    }

    //inline assembly
    let result = add(32, 54345);
    println!("{result}");
}

unsafe fn my_function() {
    println!("calling unsafe function: my_function");
}