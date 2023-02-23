fn main() {
    // Arrays
    /* An array can be defined in two ways:
     - A comma-separated list of values, where the length isn't specified.
     - The initial value followed by a semicolon, and then the array length. */

    // Declare array, initialize all values, compiler infers length = 7
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
  
    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];

    // Only one thing about an array can change over time: the values of the elements in the array.
    //The data type remains constant and the number of elements (length) remains constant. Only the values can change.

    // Get the first day of the week
    let first = days[0];

    // Get the second day of the week
    let second = days[1];

    println!("First: {}, Second: {}", first, second);
    println!("{:?}", bytes);
    println!("Length of array: {}", days.len());
}
