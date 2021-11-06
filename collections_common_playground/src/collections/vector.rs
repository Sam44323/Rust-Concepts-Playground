pub mod Vectors {
  pub fn creating_vector() -> Vec<i32> {
    let mut v = Vec::new();
    v.push(5); // pushing a new value
    v.push(6);
    v.push(7);
    v.push(10);
    println!("{:?}", v);
    v.pop(); // removing the last value
    println!("{:?}", v);
    v
  }
}
