// unsafe ability 1
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
}

fn main() {
    dereference_raw_pointers();
}
