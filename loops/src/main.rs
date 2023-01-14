fn main() {
    let mut some_number = 0;

    let result = loop {
        some_number += 1;

        if some_number == 10 {
            break some_number / 2;
        }

        println!("again!")
    };

    println!("The outcome of loop expression is: {result}");
}
