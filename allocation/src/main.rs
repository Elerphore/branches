fn main() {
    let some_string = String::from("haha go brrr");
    allocation(&some_string);

    let some_allocated_string = some_string;

    println!("{}", some_allocated_string);
}

fn allocation(some_string: &String) {
    println!("{}", some_string);
}