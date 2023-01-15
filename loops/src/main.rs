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

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 5 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let array = [10, 20, 30, 40];

    for element in array {
        println!("the value of element is {element}");
    }

    for number in (1..4).rev() {
        println!("number = {number}");
    }
}
