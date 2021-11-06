// Simple match flows example

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Nested value matching example

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Florida,
}

enum CoinNew {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn nested_matching_example(coin: CoinNew) -> Option<u8> {
    match coin {
        CoinNew::Penny => {
            println!("Lucky penny!");
            return Some(1);
        }
        CoinNew::Nickel => Some(5),
        CoinNew::Dime => Some(10),
        CoinNew::Quarter(state) => match state {
            UsState::Alabama => Some(25),
            UsState::Alaska => Some(50),
            UsState::Florida => Some(100),
        },
    }
}
fn main() {
    value_in_cents(Coin::Penny);
    println!(
        "{:?}",
        nested_matching_example(CoinNew::Quarter(UsState::Alabama))
    );
}
