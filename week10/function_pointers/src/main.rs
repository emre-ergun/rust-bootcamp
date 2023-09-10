fn main() {
    let ten = 10;
    let greater_than = |x: &i32| { *x > ten };
    let less_than = |x: &i32| { *x < 20 };

    let a_value = 15;
    let true_or_false = are_both_true(greater_than, less_than, &a_value);
    println!("{true_or_false}");

    // instead of closure less_than we can use less_than1 function instead.
    let true_or_false = are_both_true(greater_than, less_than1, &a_value);
    println!("{true_or_false}");

}

fn less_than1(value: &i32) -> bool {
    *value < 14
}

fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool
where T: Fn(&V) -> bool, U: Fn(&V) -> bool {
    f1(item) && f2(item)
}

// fn are_both_true<V>(f1: fn(&V) -> bool, f2: fn(&V) -> bool, item: &V) -> bool {
//     f1(item) && f2(item)
// }