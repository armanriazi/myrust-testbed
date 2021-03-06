extern crate rand; // <1>
use rand::*;
static mut ERROR: isize = 0;

fn one_in(n: u32) -> bool { // <3>
  ////rand::thread_rng().gen_weighted_bool(n)
  ////ThreadRng::default().gen_bool(n as f64)
  //let mut rng = thread_rng();
  //rng.gen_weighted_bool(n)
  true
}

#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
}

impl File {
  fn new(name: &str) -> File {
    File { name: String::from(name), data: Vec::new() } // <4>
  }

  fn new_with_data(name: &str, data: &Vec<u8>) -> File {
    let mut f = File::new(name);
    f.data = data.clone();
    f
  }

  fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> { // <5>
    let mut tmp = self.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    Ok(read_length)  // <6>
  }

  #[allow(unused_variables)]
   fn read_by_err(f: &File, save_to: Vec<u8>) -> usize {
   // if rand::thread_rng().gen_weighted_bool(10000) {
        unsafe {
            ERROR = 1;
        }
    //}

    0 // <> Always read() 0 bytes  
}
}

fn open(f: File) -> Result<File, String> {
  if one_in(10_000) {  // <7> 
    let err_msg = String::from("Permission denied");
    return Err(err_msg);
  }
  Ok(f)
}

fn close(f: File) -> Result<File, String> {
  if one_in(100_000) { // <8>
    let err_msg = String::from("Interrupted by signal!"); 
    return Err(err_msg);
  }
  Ok(f)
}

fn main() {
  let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
  let mut f4 = File::new_with_data("4.txt", &f4_data); 
//f4 is now rebound after each call to open() and close().
  let mut buffer: Vec<u8> = vec![];

  f4 = open(f4).unwrap();                        // <9>
  let f4_length = f4.read(&mut buffer).unwrap(); // <9>
  f4 = close(f4).unwrap();                       // <9>
  
  let text = String::from_utf8_lossy(&buffer);

  unsafe {
    if ERROR != 0 {
        panic!("An error has occurred!")
    }
}

  println!("{:?}", f4);
  println!("{} is {} bytes long", &f4.name, f4_length);
  println!("{}", text);
}