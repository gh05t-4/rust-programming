fn main() {
    println!("Hello, world!");
    goodbye();

    let formal = "Formal: Goodbye.";
    let casual = "Casual: See you later!";
    goodbye_msg(formal);
    goodbye_msg(casual);

    let num = 25;
    println!("\n{} divided by 5 = {}", num, divide_by_5(num));

    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
}

// Function definitions in Rust start with the fn keyword.
fn goodbye() {
    println!("Goodbye.");
}

// Pass input arguments
fn goodbye_msg(message: &str) {
    println!("\n{}", message);
}

// Return a value
// When a function returns a value, we add the syntax -> <type> after the list of function arguments and
// before the opening curly bracket for the function body. The arrow syntax -> indicates that the function returns a value to the caller. 
// The <type> portion lets the compiler know the data type of the value returned.

// In Rust, the common practice is to return a value at the end of a function by having the last line of code in the function be equal to the value to return.
// We can use the 'return' keyword at any point in the function to halt execution and send a value back to the caller.
fn divide_by_5(num: u32) -> u32 {
    // When you explicitly use the return keyword, you end the statement with a semicolon.
    // If you send back a return value without using the return keyword, you don't end the statement with a semicolon.
    if num == 0 {
        return 0;
    }
    num / 5
}

// EXERCISE
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {

    // Use the values of the input arguments
    // All new cars always have zero mileage
    Car { color: color, transmission: transmission, convertible: convertible, mileage: 0 }
}