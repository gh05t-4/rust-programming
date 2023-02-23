// Unit tests

/* 
Unit tests in Rust are simple functions marked with the #[test] attribute that verify that the non-test code is functioning in the expected
manner. These functions are only compiled when testing code.

Test functions run the code that you want to test. Then they check the results, often by using the assert! or assert_eq! macros.
*/

// A simple add function and another add_works function marked with the #[test] attribute.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn add_works() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(10, 12), 22);
    assert_eq!(add(6, -2), 4);
}

/* 
When we execute the command $ cargo test, our output would look like the following example:
Output

running 1 test
test add_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
*/

// Test failures
#[test]
fn add_fails() {
    assert_eq!(add(2, 2), 7);
}

// Expected failures
// The should_panic attribute makes it possible to check for a panic!. If we add this attribute to our test function, the test passes when the code in the function panics.
// The test fails when the code doesn't panic.
#[test]
#[should_panic]
fn add_fails1() {
    assert_eq!(add(2, 2), 7);
}

// Ignore tests
// A function annotated with the [test] attribute can also be annotated with the [ignore] attribute. This attribute causes that test function to be skipped during tests.

// The [ignore] attribute may optionally be written with a reason why the test is ignored.
#[test]
#[ignore = "not yet reviewed by the Q.A. team"]
fn add_negatives() {
    assert_eq!(add(-2, -2), -4)
}

// The test module
/* 
The cfg attribute controls conditional compilation and will only compile the thing it's attached to if the predicate is true.
 The test compilation flag is issued automatically by Cargo whenever we execute the command $ cargo test, so it will always be
 true when we run our tests.

The use super::*; declaration is necessary for the code inside the add_function_tests module to access the add in the outer module.

#[cfg(test)]
mod add_function_tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 2), 7);
    }

    #[test]
    #[ignore]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4)
    }
}

*/

fn main() {
    println!("2 + 2 = {}", add(2, 2));
}