use std::io::Error;
fn main() -> Result<(), Error>{
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap(), 2);
    let x: Result<u32, &str> = Err("emergency failure");
    x.unwrap(); // panics with `emergency failure`    
    Ok(())
}
