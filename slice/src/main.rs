fn main() {
    let words: String = String::from("Hello, world!");

    println!("{}", first_word(&words));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
