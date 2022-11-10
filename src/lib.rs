pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Default for Post {
  fn default() -> Self {
    Self::new()
  }
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
    self.state.as_ref().unwrap().content(self)
  }

  pub fn request_review(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review())
    }
  }

  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }

  pub fn reject(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.reject())
    }
  }
}

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn reject(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&'a self, post: &'a Post) -> &str {
    ""
  }
}

struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview { count: 0 })
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn reject(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

struct PendingReview {
  count: u32,
}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn reject(self: Box<Self>) -> Box<dyn State> {
    Box::new(Draft {})
  }

  fn approve(mut self: Box<Self>) -> Box<dyn State> {
    if self.count == 0 {
      self.count += 1;
      self
    } else {
      Box::new(Published {})
    }
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

  fn content<'a>(&'a self, post: &'a Post) -> &str {
    &post.content
  }

  fn reject(self: Box<Self>) -> Box<dyn State> {
    self
  }
}
