/*
  A trait tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

  Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
*/

// declaring a trait

fn normal_trait_implementation() {
  struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }

  struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
  }

  pub trait Summary {
    fn summarize(&self) -> String;
  }

  // example of implementing a trait on a type
  impl Summary for NewsArticle {
    fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
  }

  impl Summary for Tweet {
    fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
    }
  }

  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  // calling the methods from traits
  println!("1 new tweet: {}", tweet.summarize());
}

pub fn traits_methods_caller() {
  normal_trait_implementation();
}
