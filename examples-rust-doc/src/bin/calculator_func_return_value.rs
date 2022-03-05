fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
   return (x - y);
}
fn multi(x: u16, y: u64) -> u16 {
    x * y as u16
}
fn main() {
    println!("42 + 13 = {}", add(42, 13));
    println!("42 - 13 = {}", subtract(42, 13));
    println!("42 * 13 = {}", multi(42, 13));
}
