#[derive(PartialEq)] // <1>
struct Hostname(String);
 
fn connect(host: Hostname) {
  println!("connected to {}", host.0);
}
 
fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname ( ordinary_string.clone() );
 
    connect(host);
}