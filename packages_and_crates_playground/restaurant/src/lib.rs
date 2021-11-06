mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to wailist!");
        }

        fn seat_at_table() {
            println!("Booked a seat at table!");
        }

        pub fn exit_the_restraunt(){
            println!("Exiting the restraunt!");
        }
    }

    pub mod serving {
        fn take_order() {
            println!("Order taking in process!");
        }

        fn serve_order() {
            println!("Order serving in process!");
        }

        pub fn take_payment() {
            println!("Payment taking in process!");
            super::hosting::exit_the_restraunt(); // example of using the super keyword for calling a function from ancestor module
        }
    }
}


pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}