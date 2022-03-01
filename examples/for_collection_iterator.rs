
 fn main() {
    let a=[0..5];
    //collection.iter_mut()
    //collection.iter()
    //IntoIterator::into_iter(a)
    for item in 0..5{
      for _ in 0..10 {
        println!("{}", item);
      }    
    }
    let cl = [1, 2, 3, 4, 5];
    for i in 0..cl.len() {
    let item = cl[i];
    println!("--{}", item);
    }

    for n in 0..10 {
      if n % 2 == 0 {
        continue;
      }
      println!("**{}", n);
    }

  }