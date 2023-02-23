fn main() {
    if 1 == 2 {
        println!("True, the numbers are equal."); // 
    } else {
        println!("False, the numbers are not equal.");
    }

    // Unlike most other languages, if blocks in Rust can also act as expressions.
    // All execution blocks in the condition branches must return the same type for the code to compile.
    let formal = true;
    let greeting = if formal { // if used here as an expression
        "Good day to you."     // return a String
    } else {
        "Hey!"                 // return a String
    };
    println!("{}", greeting);   // prints "Good day to you."

    // Combine multiple test conditions
    let num = 500; // num variable can be set at some point in the program
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
    println!("Is {} out of range? {}", num, out_of_range);
}
