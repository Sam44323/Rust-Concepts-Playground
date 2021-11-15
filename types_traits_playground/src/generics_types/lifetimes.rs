fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn lifetimes_method_callers() {
  let string1 = String::from("longest");
  let string2 = String::from("short");
  let result = longest(&string1, &string2);
  println!("The longest string is {}", result);
  println!("---------------------------------");
  let another_result = single_lifetimes(&string1, &string2);
  println!("The static string renderer is:  {}", another_result);
  println!("---------------------------------");
  struct_lifetimes();
}

// example single lifetime
fn single_lifetimes<'a>(x: &'a str, y: &str) -> &'a str {
  x
}

// example of using structs with lifetimes

struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn struct_lifetimes() {
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
    part: first_sentence,
  };
  println!("Structs using lifetimes: {}", i.part);
}

// example of using the static lifetime. It means the reference can live as long as the duration of the program.
fn static_lifetime() {
  let s: &'static str = "I have a static lifetime.";
  println!("Static lifetime: {}", s);
}

pub fn lifetimes_method_caller() {
  lifetimes_method_callers();
  static_lifetime();
}
