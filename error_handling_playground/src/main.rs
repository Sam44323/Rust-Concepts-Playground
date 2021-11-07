mod errors;
use errors::panic_modules_method_caller;
use errors::result_modules_method_caller;

fn main() {
    panic_modules_method_caller();
    result_modules_method_caller();
}
