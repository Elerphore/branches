use crate::Coin::{Nickel, Penny};

enum Option<T> {
    None,
    Some(T),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('h');
    let absent_number: Option<i32> = Option::None;

    dbg!(value_in_cents(Nickel));
}
