fn main() {
    let s = String::from("Hello, world!");
    process(s); // Ownership of the string in `s` moved into `process`
    // process(s); // Error! ownership already moved.
    caller();

    // Borrowing
    let greeting = String::from("Hello");
    print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again

    // Mutate borrowed values
    let mut greeting1 = String::from("hello");
    change(&mut greeting1);

    /* 
    NOTE:
    Your code must implement either of the following definitions, but not both at the same time:
        - One or more immutable references (&T)
        - Exactly one mutable reference (&mut T)

    */

    // LIFETIMES
    let magic1 = String::from("abracadabra!");
    let result;
    let magic2 = String::from("shazam!");
    result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);
}

fn process(_input: String) {}

fn print_greeting(message: &String) {
    println!("Greeting: {}", message);
}

fn change(text: &mut String) {
    text.push_str(", world");
    println!("Mutated borrowed string is '{}'", text)
}

fn caller() {
    let s = String::from("Hello, world!");
    process(s.clone()); // Passing another value, cloned from `s`.
    process(s); // s was never moved and so it can still be used.
}

// the input parameter's lifetime relates to return value's lifetime by the named lifetime parameter ('a)
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}