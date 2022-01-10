/**
- "*" is dereferencing that is getting the value that is being references using "&"
* Diff between dereferencing_runner and box_dereferencing_runner:
*
* The only difference is that here we set y to be an instance of a box pointing to a copied value of x rather than a reference pointing to the value of x. In the box_dereferencing_runner assertion, we can use the dereference operator to follow the box’s pointer in the same way that we did when y was a reference. Next, we’ll explore what is special about Box<T> that enables us to use the dereference operator by defining our own box type.
*/

fn dereferencing_runner() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn box_dereferencing_runner() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn custom_pointer_initialization_for_Box() {}

fn main() {
    dereferencing_runner();
    box_dereferencing_runner();
    custom_pointer_initialization_for_Box();
}
