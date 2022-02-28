fn main() {
    let a: i32 = 10;
    let b: u16 = 100;
    let c: i32 = 950;   
    println!("{}",c as u8);

    //b as i32
    /*if a < b {
      println!("Ten is less than one hundred.");
    }*/



  }
  // TO_REPORT_BUG:Using type casts carelessly will cause your program to behave unexpectedly. For example, the expression 300_i32 as i8 returns 44. *BecauseOf(-): *BecauseOf(?): using as
    //300-128=172-128=44