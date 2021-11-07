use std::collections::HashMap;

pub mod HashMaps {
  pub fn create_hashmap() {
    let mut scores = super::HashMap::new(); // creating a hashmap

    // inserting values to hashmap

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
  }

  pub fn collect_hashing() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // Using the way of constructing a hash map
    // is by using iterators and the collect method on a vector of tuples,
    //which is a collection of key-value pairs.
    let scores: super::HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
  }

  pub fn method_callers() {
    collect_hashing();
  }
}
