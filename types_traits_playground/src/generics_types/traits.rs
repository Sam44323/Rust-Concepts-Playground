/*
  A trait tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

  Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
*/

// declaring a trait

fn trait_implementation() {
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

    // added a default implementation
    fn message(&self) -> String {
      String::from("How you doi'n reader?")
    }
  }

  pub trait TitleReloader {
    fn title_reloader(&self) {}
  }

  impl TitleReloader for NewsArticle {
    fn title_reloader(&self) {
      println!("Title reloaded!: {}", self.headline);
    }
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
  // this parameter accepts any type that implements the Summary trait
  fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
  }

  // example of using multiple trait bound parameters
  fn title_notifier(item: &(impl Summary + TitleReloader)) {
    println!("Breaking news! {}", item.summarize());
    item.title_reloader();
  }

  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  let news = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
    ),
  };

  // calling the methods from traits
  println!("1 new tweet: {}", tweet.summarize());
  println!("Message: {}", tweet.message());
  notify(&tweet);
  title_notifier(&news);
}

pub fn traits_methods_caller() {
  trait_implementation();
}
