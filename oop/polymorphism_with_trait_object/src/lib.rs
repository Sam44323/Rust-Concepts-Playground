pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>, // dyn Draw means our Box pointer will contain any type that contains the Draw trait
}
