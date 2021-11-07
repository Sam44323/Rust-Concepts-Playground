/*
  A trait tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

  Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
*/

// declaring a trait

pub trait Summary {
  fn summarize(&self) -> String;
}

pub fn traits_methods_caller() {}