struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("username"),
        email: String::from("email@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1

    };

    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);

    let subject = AlwaysEqual;

    user1.email = String::from("anotheremail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}