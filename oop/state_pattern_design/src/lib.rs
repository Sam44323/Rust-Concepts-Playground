trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content(&self, post: &Post) -> &str;
}

struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    return Box::new(PendingReview {});
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn content(&self, post: &Post) -> &str {
    ""
  }
}

struct PendingReview {}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    return Box::new(Published {});
  }

  fn content(&self, post: &Post) -> &str {
    ""
  }
}

struct Published {}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn content(&self, post: &Post) -> &str {
    post.content
  }
}

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

  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn content(&self) -> &str {
    /*
     *  as state is an option with boxed state that owns the state object, so we use as_ref to get the reference to the state object along with that we need to unwrap it to get the state
     */
    self.state.as_ref().unwrap().content(self);
  }

  pub fn request_review(&mut self) {
    // take method takes the inner value out of the Option and places none so that when we call the request_review method we can call the method with the state and get the output that is desired
    if let Some(state) = self.state.take() {
      self.state = Some(state.request_review());
    }
  }

  pub fn approve(&mut self) {
    if let Some(state) = self.state.take() {
      self.state = Some(state.approve());
    }
  }
}
