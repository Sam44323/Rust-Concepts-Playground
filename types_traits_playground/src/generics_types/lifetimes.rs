fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

// example single lifetime
fn single_lifetimes<'a>(x: &'a str, y: &str) -> &'a str {
  x
}

fn longest_lifetimes() {
  let string1 = String::from("longest");
  let string2 = String::from("short");
  let result = longest(&string1, &string2);
  println!("The longest string is {}", result);
  println!("---------------------------------");
  let another_result = single_lifetimes(&string1, &string2);
  println!("The static string renderer is:  {}", another_result);
  println!("---------------------------------");
}

pub fn lifetimes_method_caller() {
  longest_lifetimes();
}
