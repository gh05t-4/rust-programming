struct Point<T> {
    x: T,
    y: T,
}

struct Point1<T, U> {
    x: T,
    y: U,
}

fn main() {
    // A generic data type is a type that's defined in terms of other, partially unknown types.

    // To implement a new generic type, we must declare the name of the type parameter inside angle brackets just after the name of the struct.
    // Then we can use the generic type in the struct definition where we would otherwise specify concrete data types.
    let _boolean = Point { x: true, y: false };
    let integer = Point { x: 1, y: 9 };
    let _float = Point { x: 1.7, y: 4.3 };
    let _string_slice = Point { x: "high", y: "low" };
    println!("Point x: {}, Point y: {}", integer.x, integer.y);

    // NOTE:
    // Even though T can assume any concrete type, x and y must be of that same type, because they were defined as being of the same type.
    // If we try to create an instance of a Point<T> that has values of different types, as in the following snippet, our code won't compile.
    // let wont_work = Point { x: 25, y: true }; // This will error as mismatched types.

    // we can use multiple generic type parameters.
    // In this case, we show a Point<T, U> generic over two types so that x and y can be values of different types.
    let _integer_and_boolean = Point1 { x: 5, y: false };
    let _float_and_string = Point1 { x: 1.0, y: "hey" };
    let integer_and_float = Point1 { x: 5, y: 4.0 };
    let _both_integer = Point1 { x: 10, y: 30 };
    let _both_boolean = Point1 { x: true, y: true };
    println!("Point x: {}, Point y: {}", integer_and_float.x, integer_and_float.y);
}
