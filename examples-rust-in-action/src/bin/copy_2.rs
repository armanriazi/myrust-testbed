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

impl Copy for CubeSat { }

 
impl Clone for CubeSat {
  fn clone(&self) -> Self {
    CubeSat { id: self.id }
    //*self
  }
}
  
/*impl Copy for StatusMessage { }
impl Clone for StatusMessage {
  fn clone(&self) -> Self {
    *self
  }
}*/
 
fn main() {
  let sat_a = CubeSat { id: 0 };
 
  let a_status = check_status(sat_a);
  println!("a: {:?}", a_status);
 
  let a_status = check_status(sat_a);
  println!("a: {:?}", a_status);
}