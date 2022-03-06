fn dead_end() -> ! {
    panic!("you have reached a dead end");
  }

  fn main(){
      dead_end();
  }