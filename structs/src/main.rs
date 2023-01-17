struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user = build_user(String::from("some_funny_email@gmail.com"), String::from("really_cool_username"),);

    let user_2 = User {
        email: String::from("Oh.No.@gmail.com"),
        ..user
    };

    let color_1 = Color(10, 10, 10);

    println!("{}", String::from(user));
}
