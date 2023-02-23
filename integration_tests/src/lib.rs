/* 
To test our crate as a whole, we can use integration tests. The Rust test suite supports this type of test, which only calls functions
that our library's public API contains.

When you run integration tests with Cargo, put them in a tests directory. Cargo runs each source file in this directory.
Create tests in your project directory, at the same level as your src directory.

Only library crates can be tested via integration tests because binary crates don't expose any functionality that other crates can use.
*/

pub struct Pizza {
    pub topping: String,
    pub inches: u8,
}

impl Pizza {
    pub fn pepperoni(inches: u8) -> Self {
        Pizza::bake("pepperoni", inches)
    }

    pub fn mozzarella(inches: u8) -> Self {
        Pizza::bake("mozzarella", inches)
    }

    fn bake(topping: &str, inches: u8) -> Self {
        Pizza {
            topping: String::from(topping),
            inches,
        }
    }
}