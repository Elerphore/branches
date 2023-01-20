use crate::Coin::{Nickel, Penny, Dime, Quarter};
use crate::Option::{None, Some};

#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Penny => {
            println!("lucky penny");
            1
        },
        Nickel => 5,
        Dime => 10,
        Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('h');
    let absent_number: Option<i32> = Option::None;

    dbg!(value_in_cents(Quarter(UsState::Alabama)));

    dbg!(plus_one(Some(1)));
}
