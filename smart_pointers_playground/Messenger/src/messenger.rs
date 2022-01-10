pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}
