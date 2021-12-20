fn iterator_example() {
    let v1 = vec![1, 2, 3];

    for val in v1 {
        println!("Got: {}", val);
    }
}

/**
 * Note regarding Item:
 *For now, all you need to know is that this code says implementing the Iterator trait requires that you also define an Item type, and this Item type is used in the return type of the next method. In other words, the Item type will be the type returned from the iterator.
 */

fn iterator_trait_implementation() {
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }
}
fn iterator_deepdowns() {
    /*
    * Note that we needed to make v1_iter mutable: calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. In other words, this code consumes, or uses up, the iterator. Each call to next eats up an item from the iterator. We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership of v1_iter and made it mutable behind the scenes.

    Also note that the values we get from the calls to next are immutable references to the values in the vector. The iter method produces an iterator over immutable references. If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter. Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.
    */

    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn main() {
    iterator_example();
    iterator_trait_implementation();
    iterator_deepdowns();
    println!("Hello, world!");
}
