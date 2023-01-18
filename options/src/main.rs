enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('h');
    let absent_number: Option<i32> = Option::None;
}
