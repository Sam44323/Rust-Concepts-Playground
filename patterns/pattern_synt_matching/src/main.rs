fn main() {
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
}
