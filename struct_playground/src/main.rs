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
/*
 * Example of a Tuple Struct
 */

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/*
 * Example of a Unit Struct
 */

struct Unit;

fn struct_basics_runner() {
    /*
     * Instantiating a struct example. For this case, we are instantiating User
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

    /*
     *  Example of instantiating a tuple struct
     */

    let color = Color(0, 0, 0);
    let origin = Point(10, 14, 15);

    // Accessing a tuple struct field
    println!(
        "Color struct are as follows: {} {} {}",
        color.0, color.1, color.2
    );
    println!(
        "Origin struct are as follows: {} {} {}",
        origin.0, origin.1, origin.2
    );

    // Example of instantiating a unit struct
    let unit = Unit;
}

/*
    Creating a program that calculates the area of a rectangle. Here we'll be implementing Tuples for practice.
*/

fn area_tuples(dimensions: &(i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

fn calculate_area_tuples() {
    let rect1 = (30, 50);
    println!("The area of the rectangle is {}", area_tuples(&rect1));
}

/*
    Creating a program that calculates the area of a rectangle. Here we'll be implementing Struct for practice.
*/

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn area_struct(dimensions: &Rectangle) -> i32 {
    dimensions.width * dimensions.height
}

fn calculate_area_struct() {
    let rect1 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("The area of the rectangle is {}", area_struct(&rect1));
    println!("The rect1 value is {:?}", rect1);
    dbg!(&rect1); // stderr debugger example
}

/*
Example of implementing methods for structs. Here we'll be using the same rectangle area example but with diff names
*/

#[derive(Debug)]
struct RectangleNew {
    width: u32,
    height: u32,
}

// implementing the area method for the RectangleNew struct
impl RectangleNew {
    // adding an associated function which acts as constructor
    fn rectangle(size: u32) -> RectangleNew {
        RectangleNew {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// example of multiple implementation blocks
impl RectangleNew {
    // example of implementation method for the RectangleNew struct taking multiple params
    fn can_hold(&self, other: &RectangleNew) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn calculate_area_struct_methods() {
    let rect_a = RectangleNew {
        width: 10,
        height: 40,
    };

    let rect_b = RectangleNew {
        width: 5,
        height: 30,
    };

    let rect_c = RectangleNew::rectangle(13);

    println!("The area of the rectangle is {}", rect_a.area());
    println!("Can rect_a hold rect_b? {}", rect_a.can_hold(&rect_b));
    println!("Can rect_b hold rect_a? {}", rect_b.can_hold(&rect_a));
    println!(
        "The area of the rectangle created out of associated function as a constructor is {}",
        rect_c.area()
    );
}

fn main() {
    // basic
    struct_basics_runner();

    //code examples
    calculate_area_tuples();
    calculate_area_struct();

    // example for method implementation on structs
    calculate_area_struct_methods();
}
