use super::hosting::Hosting::exit_the_restraunt;

pub mod Serving {
  fn take_order() {
    println!("Order taking in process!");
  }

  fn serve_order() {
    println!("Order serving in process!");
  }

  pub fn take_payment() {
    println!("Payment taking in process!");
    exit_the_restraunt();
  }
}
