fn main() {

    let mut user1 = User {
        email: String::from("someone@awesome.com"),
        username: String::from("someonegreat"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    user1.email = String::from("someone@else.com");

    println!("{}", user1.email);

    let user2 = User {
        email: String::from("another@awesesome.com"),
        username: String::from("anothergreat"),
        ..user1
    };

    // tuple structs

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0,0,0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}