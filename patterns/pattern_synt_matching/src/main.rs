fn main() {
    // using the Match Expressions
    // ------------------------------------------------
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
        _ => println!("Japanese"),
    }

    // ------------------------------------------------
}
