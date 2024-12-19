struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

fn main() {
    let user1_email = String::from("someone@example.com");
    let user1_username = String::from("someusername123");


    let user1 = build_user(user1_email, user1_username);
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    let user3 = User{
        email: String::from("aThird@example.com"),
        ..user2
    };

    println!("{}", user2.email);
    println!("{}", user3.email);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
