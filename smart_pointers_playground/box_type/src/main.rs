/*
example of using the cons list
Note:
-------
A cons list is a data structure that comes from the Lisp programming language and its dialects. In Lisp, the cons function (short for “construct function”) constructs a new pair from its two arguments, which usually are a single value and another pair. These pairs containing pairs form a list.

The cons function concept has made its way into more general functional programming jargon: “to cons x onto y” informally means to construct a new container instance by putting the element x at the start of this new container, followed by the container y.

Each item in a cons list contains two elements: the value of the current item and the next item. The last item in the list contains only a value called Nil without a next item. A cons list is produced by recursively calling the cons function. The canonical name to denote the base case of the recursion is Nil. Note that this is not the same as the “null” or “nil”.

Although functional programming languages use cons lists frequently, the cons list isn’t a commonly used data structure in Rust. Most of the time when you have a list of items in Rust, Vec<T> is a better choice to use.

We are using Box<List> because it solves the issue for
Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to. This means we can put a Box<T> inside the Cons variant instead of another List value directly. The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant. Conceptually, we still have a list, created with lists “holding” other lists, but this implementation is now more like placing the items next to one another rather than inside one another.

Reason for box to be smart-pointer:
The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references. When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation. Let’s explore these two traits in more detail
*/

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn cons_list_runner() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn box_initializer() {
    let b = Box::new(5);
    println!("Box: {}", b);
}

fn main() {
    box_initializer();
    cons_list_runner();
}
