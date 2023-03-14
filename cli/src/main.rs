mod l1;
mod l2;
mod sub1;
mod sub2;
mod sub3;
use crate::{sub1::minus, sub2::mul_mul};
use l1::div_div;
use lib1::add_add;

fn main() {
    println!("Hello, world!");
    println!("{}", lib1::add(1, 2));
    println!("{}", add_add::add2(1, 2));
    println!("{}", minus(3, 2));
    println!("{}", mul_mul(3, 2));
    println!("{}", div_div::div_div(3, 2));
    println!("{}", div_div::div_power(3, 2));
}
