mod generics_types;
use generics_types::generic_method_caller;
use generics_types::lifetimes_method_caller;
use generics_types::traits_methods_caller;

fn main() {
    generic_method_caller();
    traits_methods_caller();
    lifetimes_method_caller();
}
