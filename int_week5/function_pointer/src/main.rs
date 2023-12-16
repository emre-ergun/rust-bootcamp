fn main() {
    let ten = 10;
    // this is not coerced because it is capturing ten variable from the environemnt
    let greater_than1 = |x: &i32| *x > ten; 
    println!("Are both true?: {}", are_both_true(greater_than, less_than, &15));
}

fn less_than(x: &i32) -> bool {
    *x < 20
}

fn greater_than(x: &i32) -> bool {
    *x > 10
}
fn are_both_true<V>(f1: fn(&V)->bool, f2: fn(&V)->bool, item: &V) -> bool {
    f1(item) && f2(item)
}
