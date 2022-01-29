// unsafe ability a
fn dereference_raw_pointers() {
    let mut num = 5;

    /*
     * Properties of raw pointers are as follows:
     * 1. They are also not guaranteed yo point to point to valid data or memory
     *
     * 2. They are allowed to ignore rust borrowing rules by having mutable and immutable pointers or multiple immutable pointers to the same location
     *
     * 3. They are allowed to be null
     *
     * 4. They don't implement automatic cleanup which mean you can have dangling pointers
     */

    // here * is not dereference but it's simply how we declare raw pointers in rust. Here we are creating immutable and mutable raw pointers from references

    let a = &num as *const i32; // immutable raw pointer example is *const <type point to>
    let b = &mut num as *mut i32; // immutable raw pointer example is *mut <type point to>

    // dereferencing raw pointers using unsafe block

    unsafe {
        println!("a is: {}", *a);
        println!("b is: {}", *b);
    }
}

// unsafe ability b
fn calling_unsafe_functions() {
    unsafe fn dangerous() {
        println!("I am unsafe function!");
    }

    unsafe {
        dangerous();
    }
}

// unsafe ability c

fn safe_abstraction_creator_over_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5];
    let a = &mut v[..];

    let (x, y) = a.split_at_mut(3);

    assert_eq!(x, &mut [1, 2, 3]);
    assert_eq!(y, &mut [4, 5]);
}

// unsafe ability c

// extern helps you to declare functions that are implemented in other languages or FFI

extern "C" {
    fn abs(input: i32) -> i32; // the interface of the external function in another language that we want to call and rust then maps it to the ABI
}

fn calling_external_functions() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// unsafe ability d

static mut COUNTER: i32 = 0; // const as equal

fn modify_mutable_static_variable() {
    unsafe {
        COUNTER += 1;
    }
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn main() {
    dereference_raw_pointers();
    calling_unsafe_functions();
    safe_abstraction_creator_over_unsafe_code();
    calling_external_functions();
    modify_mutable_static_variable();
}
