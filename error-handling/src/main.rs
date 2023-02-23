fn main() {
    // You can use the panic! macro to panic the current thread.
    // The macro prints an error message, frees resources, and then exits the program.
    /* 
    panic!("Farewell!");
    /* 
    OUTPUT:
      thread 'main' panicked at 'Farewell!', src/main.rs:4:5
    */
    */

    // Option<T> enum
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    // NOTE: how can we access the data inside a Some(data) variant?
    // Pattern matching
    for &index in [0, 2, 4, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    // The if let expression

    // An if let operator compares a pattern with an expression.
    // If the expression matches the pattern, the if block is executed.
    // The nice thing about if let expressions is that you don't need all the boilerplate code of a match expression when you're interested in a single pattern to match against.

    let a_number: Option<u8> = Some(7);
    if let Some(7) = a_number {
        println!("That's my lucky number!");
    }

    // unwrap and expect

    // You can try to access the inner value of an Option type directly by using the unwrap method.
    // Be careful, though, because this method will panic if the variant is a None.
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    /* let empty_gift: Option<&str> = None;
    assert_eq!(empty_gift.unwrap(), "candy"); // This will panic! */

    // The expect method does the same as unwrap, but it provides a custom panic message that's provided by its second argument
    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");

    /* let b: Option<&str> = None;
    b.expect("fruits are healthy"); // panics with `fruits are healthy` */
    
    /* 
    NOTE:
     - Use pattern matching and handle the None case explicitly.
     - Call similar non-panicking methods, such as unwrap_or, which returns a default value if the variant is None or the inner value if the variant is Some(value).
    */

    // Use the Result type to handle errors
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}

#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}