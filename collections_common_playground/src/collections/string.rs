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
                       // for &s2, the compiler coerces the &String to a &str
    println!("Concatenated String: {}", s3);
  }

  pub fn format_macro() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! macro returns a String
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Formatted String: {}", s);
  }

  pub fn slicing_strings() {
    let s = String::from("hello world");
    let hello = &s[0..5]; // slicing a string
    let world = &s[6..11];
    println!("First 5 characters: {}", hello);
    println!("Last 5 characters: {}", world);

    // slicing specific characters words. Here for "Здравствуйте" each contains two-bytes, so the match will be little different
    let hello = "Здравствуйте";

    let slice = &hello[0..4];
    println!("First 4 characters: {}", slice); // answer will be "Зд"(told ya! two bytes)
  }

  pub fn method_callers() {
    updating_strings();
    concatenation();
    format_macro();
    slicing_strings();
  }
}
