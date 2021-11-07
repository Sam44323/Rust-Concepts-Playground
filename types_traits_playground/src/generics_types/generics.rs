/*
Performance with generics:
----------------------------
You might be wondering whether there is a runtime cost when you’re using generic type parameters. The good news is that Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

In this process, the compiler does the opposite of the steps we used to create the generic function in Listing 10-5: the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

Let’s look at how this works with an example that uses the standard library’s Option<T> enum:



let integer = Some(5);
let float = Some(5.0);

When Rust compiles this code, it performs monomorphization. During that process, the compiler reads the values that have been used in Option<T> instances and identifies two kinds of Option<T>: one is i32 and the other is f64. As such, it expands the generic definition of Option<T> into Option_i32 and Option_f64, thereby replacing the generic definition with the specific ones.
*/

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
      // creating a new Point with mixed types
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
