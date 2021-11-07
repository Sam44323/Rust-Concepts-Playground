pub fn call_panic() {
  panic!("crash and burn!");
}

pub fn panic_modules_method_caller() {
  call_panic();
}
