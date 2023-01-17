mod assignments;

use crate::assignments::assignment_one::gcd;

fn main() {
    gcd::gcd().expect("panic");
    println!("Hello, world!");
}
