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
    /*
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

    /*
     * Example of creating a new struct out of another struct.
     */

    /*
     * Note:
     *  In this example, we can no longer use user1 after creating user_b because the String in the username field of user_a was moved into user_b. If we had given user_b new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user_a would still be valid after creating user_b. The types of active and sign_in_count are types that implement the Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.
     */
    let user_c = User {
        email: String::from("newexample@email.com"),
        username: String::from("newexample"),
        ..user_a
    };

    println!("{}", user_c.username);
}
