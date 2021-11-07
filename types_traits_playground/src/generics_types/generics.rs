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

fn method_definition_generics() {
  /*
  We can implement methods on structs and enums
  and use generic types in their definitions, too
  */
  struct Point<T> {
    x: T,
    y: T,
  }

  /*
  Note that we have to declare T just after impl so we can use it to specify that we’re implementing methods on the type Point<T>. By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
  */

  impl<T> Point<T> {
    fn x(&self) -> &T {
      &self.x
    }

    fn y(&self) -> &T {
      &self.y
    }
  }

  let p = Point { x: 5, y: 10 };
  println!("p.x = {}", p.x());
  println!("p.y = {}", p.y());
}

fn method_definition_generics_mixup() {
  /*
  We can mix up generic types and concrete types in method definitions
  */
  struct Point<T, U> {
    x: T,
    y: U,
  }

  impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
      // creating a new pointer with mixed types
      Point {
        x: self.x,
        y: other.y,
      }
    }
  }

  let p1 = Point { x: 5, y: 10.4 };
  let p2 = Point { x: "Hello", y: 'c' };

  let p3 = p1.mixup(p2);
  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

pub fn generic_method_caller() {
  println!("generic_method_caller");
  struct_generics_single_type();
  struct_generics_multiple_type();
  method_definition_generics();
  method_definition_generics_mixup();
}
