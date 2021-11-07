fn struct_generics_single_type() {
  struct Point<T> {
    x: T,
    y: T,
  }
  let integer = Point { x: 5, y: 10 };

  //let float = Point { x: 1.0, y: 4.0 }; // will give an error because of type mismatch

  println!("X: {} ; Y: {}", integer.x, integer.y);
}

fn struct_generics_multiple_type() {
  // example of declaring struct with multiple generic types

  struct Point<T, U> {
    x: T,
    y: U,
  }
  let multi_type = Point {
    x: 5,
    y: String::from("This is another type!"),
  };
  println!("X: {} ; Y: {}", multi_type.x, multi_type.y);
}

pub fn generic_method_caller() {
  println!("generic_method_caller");
  struct_generics_single_type();
  struct_generics_multiple_type();
}
