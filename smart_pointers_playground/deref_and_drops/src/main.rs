use std::ops::Deref;

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

/**
 * The Deref trait, provided by the standard library, requires us to implement one method named deref that borrows self and returns a reference to the inner data.
 *
 * The type Target = T; syntax defines an associated type for the Deref trait to use
 */

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/**
 * The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope. You can provide an implementation for the Drop trait on any type, and the code you specify can be used to release resources like files or network connections.
 */

fn custom_pointer_initialization_for_box() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn main() {
    dereferencing_runner();
    box_dereferencing_runner();
    custom_pointer_initialization_for_box();
}
