use std::convert::TryInto;
 
 fn main() {
   let a: i32 = 10;
   let b: u16 = 100;
 
   let b_ = b.try_into()
             .unwrap();
             //returns an i32 value wrapped within a Result.
             //The unwrap() method can handle the success value and returns the value of b as an i32 here
                //. If the conversion between u16 and i32 were to fail, then calling unsafe() would crash the program
   if a < b_ {
     println!("Ten is less than one hundred.");
   }
 }