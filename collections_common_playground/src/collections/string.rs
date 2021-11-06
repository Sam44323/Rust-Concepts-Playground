pub mod Strings {
  pub fn create_string() -> String {
    let s = String::from("This is string");
    /*
    Alternative ways to create a string are as follows:
    - String::new();
    - "(you string name)".to_string();
    */
    return s;
  }

  pub fn updating_strings() {
    let mut s = create_string();
    println!("Original String: {}", s);
    s.push_str(" and this is the another string"); // use push_str() to append a string to a string
    s.push('!'); // use push() to append a character to a string
    println!("Updated String: {}", s);
  }

  pub fn concatenation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used(ownership and borrowing logic)
    println!("Concatenated String: {}", s3);
  }

  pub fn method_callers() {
    updating_strings();
    concatenation();
  }
}
