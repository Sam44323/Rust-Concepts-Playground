fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn longest_lifetimes() {
  let string1 = String::from("longest");
  let string2 = String::from("short");
  let result = longest(&string1, &string2);
  println!("The longest string is {}", result);
}

pub fn lifetimes_method_caller() {
  longest_lifetimes();
}
