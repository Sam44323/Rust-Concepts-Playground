use std::fs::File;

pub fn result_error_handler() {
  let f = File::open("hello.txt");

  match f {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
  }
}

pub fn result_modules_method_caller() {
  result_error_handler();
}
