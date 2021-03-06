use std::fs::File;
use std::io::ErrorKind;

pub fn result_error_handler() {
  let f = File::open("hello.txt");

  match f {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
  }
}

pub fn error_type_handler() {
  // matching on Different Errors

  let f = File::open("hello.txt");

  match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      _ => panic!("Problem opening the file: {:?}", error),
    },
  }
}

pub fn unwrap_expect_error_handlers() {
  let f = File::open("hello.txt").unwrap(); // will throw and error if operation is not successful
  let f = File::open("hello.txt").expect("Failed to open hello.txt"); // will do same as unwrap but let us choose the panic message on error
}

pub fn result_modules_method_caller() {
  result_error_handler();
  error_type_handler();
  unwrap_expect_error_handlers();
}
