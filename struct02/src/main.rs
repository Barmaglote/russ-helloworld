struct User {
    username: String,
    email: String,
    test: String
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1: User = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        test: String::from("user1"),
    };

    let mut user2: User = User {
        username: user1.username,
        email: user1.email,
        test: String::from("user2")
    };

    user2.email = "another@gmail.com".to_string();

    println!("{}", user2.email);
    println!("{}", user1.test); // user1.email - is not available, because it is borrowed

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

