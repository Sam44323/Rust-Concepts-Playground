fn main() {
    // ------------------------------------------------
    // using the Match Expressions
    // ------------------------------------------------
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }

    let language = Language::English;

    match language {
        Language::English => println!("English"),
        Language::Spanish => println!("Spanish"),
        Language::Russian => println!("Russian"),
        lang => println!("Unsupported language: {:?}", lang),
    }

    // ------------------------------------------------
    // conditional if-let expressions
    // ------------------------------------------------

    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "38".parse();

    if let Some(auth) = authorization_status {
        println!("Authorization status: {}", auth);
    } else if is_admin {
        println!("Admin access for user");
    } else if let Ok(group) = group_id {
        if group > 30 {
            println!("Group id is greater than 30");
        } else {
            println!("Group id is less than 30");
        }
    } else {
        println!("No access for you!");
    }

    // ------------------------------------------------
    // while-let conditional expressions
    // ------------------------------------------------

    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(val) = stack.pop() {
        println!("{}", val);
    }

    // ------------------------------------------------
    // for loop expressions
    // ------------------------------------------------

    let vec: Vec<char> = vec!['a', 'b', 'c'];

    for (index, value) in vec.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
