fn struct_generics_single_type() {
  struct Point<T> {
    x: T,
    y: T,
  }
  let integer = Point { x: 5, y: 10 };
  //let float = Point { x: 1.0, y: 4.0 }; // will give an error because of type mismatch
  println!("X: {} ; Y: {}", integer.x, integer.y);
}

pub fn generic_method_caller() {
  println!("generic_method_caller");
  struct_generics_single_type();
}
