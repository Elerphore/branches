struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user = build_user(String::from("some_funny_email@gmail.com"), String::from("really_cool_username"),);

    println!("{}", String::from(user));
}
