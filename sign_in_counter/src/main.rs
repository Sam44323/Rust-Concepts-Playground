struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// example of using tuple_structs

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user: User = User {
        username: String::from("someone"),
        email: String::from("peakyblinder@email.com"),
        sign_in_count: 1,
        active: true,
    };

    user.username = String::from("Cillian");

    // inheriting some data from another user for an user

    let anotherUser: User = User {
        username: String::from("anotherUser"),
        email: String::from("anotherUser@email.com"),
        ..user
    };
}
