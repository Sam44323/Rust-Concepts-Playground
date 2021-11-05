/**
 * defining a struct
 */

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_counter: u64,
}

/**
 * creating a function for returning a struct based on the params
 */

fn build_struct(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_counter: 1,
    }
}

fn main() {
    /**
     * Instantiating a struct. For this case, we are instantiating User
     */
    let mut user_a = User {
        active: true,
        username: String::from("johndoe"),
        email: String::from("someone@email.com"),
        sign_in_counter: 1,
    };

    // Accessing a struct field
    println!("{}", user_a.username);

    // Updating a field of a struct
    user_a.username = String::from("johndoeanother");
    println!("{}", user_a.username);

    let user_b = build_struct(
        String::from("joemarkberg"),
        String::from("joemarkberg@email.com"),
    );
    println!("{}", user_b.username);
}
