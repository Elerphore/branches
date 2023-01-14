fn main() {
    let number = 7;

    let real_number = if number < 2 { 2 } else { 5 };

    if real_number < 5 {
        println!("condition was true")
    } else {
        println!("condition was false")
    }
}
