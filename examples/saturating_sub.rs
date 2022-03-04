#![allow(unused)]
#![feature(mixed_integer_ops)]
fn main() {
assert_eq!(1i32.saturating_add_unsigned(2), 3);
assert_eq!(i32::MAX.saturating_add_unsigned(100), i32::MAX);
}