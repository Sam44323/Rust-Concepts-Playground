pub struct DraftPost {
  content: String,
}

pub struct Post {
  content: String,
}

impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text)
  }

  pub fn request_review(self) -> PendingReview {
    PendingReview {
      content: self.content,
    }
  }
}

pub struct PendingReview {
  content: String,
}

impl PendingReview {
  pub fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }
}
