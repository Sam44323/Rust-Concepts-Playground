use std::collections::HashMap;

pub mod HashMaps {
  pub fn create_hashmap() {
    let mut scores = super::HashMap::new(); // creating a hashmap

    scores.insert(String::from("Blue"), 10); // inserting values to hashmap
    scores.insert(String::from("Yellow"), 50);
  }

  pub fn method_callers() {
    println!("caller");
  }
}
