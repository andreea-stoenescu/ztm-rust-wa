#![allow(non_snake_case)]
fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}
fn main() {
    let foo = add(10, 5);
    println!("{0} {0}", foo);
}
