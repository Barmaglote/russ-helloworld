struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    user1.email = String::from("anotheremail@example.com");  // Note that the entire instance must be mutable;

    println!("{}", user1.email);

    let user2 = build_user(String::from("news@example.com"), String::from("someusername123"));

    println!("{}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
