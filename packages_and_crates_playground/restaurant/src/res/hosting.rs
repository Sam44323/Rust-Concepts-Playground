use super::serving::Serving::take_payment;

pub mod Hosting {
  pub fn add_to_waitlist() {
    println!("Added to wailist!");
  }

  pub fn seat_at_table() {
    println!("Booked a seat at table!");
  }

  pub fn exit_the_restraunt() {
    println!("Exiting the restraunt!");
  }
}

fn main() {
  take_payment()
}
