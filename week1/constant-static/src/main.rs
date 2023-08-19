const MAX_PLAYER: u8 = 10;
// const variables can not be mutated.
// explicit type annotation is required for const variables.
// the value of a constant has to be computed at compile time


static CASINO_NAME: &str = "Rusty Casino";
// unlike constants, static variables are can be mutated by using "mut" keyword.
// however, accesing or modifiying value of a static variable is unsafe.
// So, this operations must be done in unsafe block.

fn main() {
    let a = MAX_PLAYER; // the value of a constant will be inlined.
    let b = MAX_PLAYER; // for these examples MAX_PLAYER will be replaced by 10.

    let c = CASINO_NAME; // static variable has an address in the memory.
    let d = CASINO_NAME;
}
