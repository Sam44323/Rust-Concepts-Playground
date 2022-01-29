pub trait Draw {
  fn draw(&self);
}

/*
* Here as the list of components can contain any type of component, that implements the Draw trait, if we would've used a generic, then we would've had the option to store only one type of component(such as buttons, slider or etc...), but not a mixture of those. So we are using dyn pointers instead of generic like,

pub struct Screen<T: Draw>{}

Trait objects in a nutshell, helps you to store any type of data that implements a certain trait thus leading to homogenous collections.

*/

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>, // dyn Draw means our Box pointer will contain any type that contains the Draw trait
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!("Drawing the button!")
  }
}
