/*fn main() {
    let x = (-42.0_f32).sqrt();
    assert_eq!(x, x);
  }*/

  fn main() {
    let x: f32 = 1.0 / 0.0;
    assert!(x.is_infinite()==true);
    assert!(x.is_finite()==false);
    assert!(x.is_nan()==false);
  }