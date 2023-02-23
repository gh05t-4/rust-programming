/* 
In Rust, all iterators implement a trait named Iterator that's defined in the standard library and is
used to implement iterators over collections such as ranges, arrays, vectors, and hash maps.
*/

/* 
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    ...
}

An Iterator has a method, next, which when called returns an Option<Item>. The next method will return Some(Item) as long
as there are elements. After they've all been exhausted, it will return None to indicate that iteration is finished.

Notice this definition uses some new syntax: type Item and Self::Item, which define an associated type with this trait.
This definition means that every implementation of the Iterator traits also requires the definition of the associated Item type,
which is used as the return type of the next method. In other words, the Item type will be the type returned from the iterator
inside the for loop block.
*/

/* 
Implement our own iterator

Creating an iterator of your own involves two steps:
   - You create a struct to hold the iterator's state.
   - You implement the iterator for that struct.
*/

// Let's make an iterator named Counter, which counts from one to an arbitrary number, that's defined when the Counter struct is created.
#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize,
}

// First, we create the struct that will hold our iterator state. We also implement a new method to control how it should be initiated.
impl Counter {
    fn new(length: usize) -> Counter {
        Counter { length, count: 0 }
    }
}

/* 
Then, we implement our Counter struct's Iterator trait. We'll be counting with usize, so we declare that our associated Item type
should be of that type.

The next() method is the only required method that we should define. Inside its body, we increment our count by one at every call
(which is why we started at zero). Then we check to see if we've finished counting or not. We use the Some(value) variant of the
Option type to express that iteration is still yielding results and the None variant to express that iteration should stop.
*/
impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // We can check that our Counter works by explicitly calling its next function.
    let mut counter = Counter::new(6);
    println!("Counter just created: {:#?}", counter);

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), Some(6));
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);  // further calls to `next` will return `None`
    assert_eq!(counter.next(), None);

    println!("Counter exhausted: {:#?}\n", counter);

    // But calling next this way gets repetitive. Rust allows us to use for loops in types that implement the Iterator trait,
    for number in Counter::new(10) {
        println!("{}", number);
    }

    // The Iterator trait's full definition includes other methods too, but they're default methods.
    // They're built on top of next, so you get them for free:
    let sum_until_10: usize = Counter::new(10).sum();
    assert_eq!(sum_until_10, 55);

    let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
    assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);
}
