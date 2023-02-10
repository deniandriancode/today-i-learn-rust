struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        username: String::from("johndoe"),
        email: String::from("john@doe.com"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = build_user(String::from("brown"), String::from("bmail@own.com"));

    println!("{}", user1.username);
    println!("{}", user2.username);

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
