//#[derive(Debug,Copy,Clone)]
#[derive(Debug)]
struct CubeSat {
  id: u64,
}
 
//#[derive(Debug,Copy,Clone)]
#[derive(Debug)]
enum StatusMessage {
  Ok,
}
 
fn check_status(sat_id: CubeSat) -> StatusMessage {
  StatusMessage::Ok
}
 
fn main() {
  let sat_a = CubeSat { id: 0 };
 
  let a_status = check_status(sat_a);
  println!("a: {:?}", a_status);
 
  /*let a_status = check_status(sat_a);
  println!("a: {:?}", a_status);*/
}