trait State {}

struct Draft {}

impl State for Draft {}

pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }

  pub fn add_text(&self, text: &str) {}

  pub fn request_review(&self) {}

  pub fn approve(&self) {}
}
