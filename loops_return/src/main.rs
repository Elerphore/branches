fn main() {
    let mut number = 0;
    let some_cool_fucking_number = loop {
        if number > 4 { break number * 2; }

        number += 1;
    };

    println!("{}", some_cool_fucking_number);
}
