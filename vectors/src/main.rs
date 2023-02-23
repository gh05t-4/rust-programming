fn main() {
    // As with arrays, vectors store multiple values that have the same data type.
    // Unlike arrays, the size or length of a vector can grow or shrink at any time.

    // When you read code in the Rust language, you'll notice the syntax <T>.
    // This syntax represents the use of a generic type T. We use a generic type declaration when we don't yet know the actual data type.
    // The syntax <vector><T> declares a vector type composed of a generic (not yet known) data type T.

    // A common way to declare and initialize a vector is with the vec! macro. This macro also accepts the same syntax as the array constructor.
    // Declare vector, initialize with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);  
    
    // Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    // Vectors can also be created by using the Vec::new() method.
    // This method of vector creation lets us add and remove values at the end of the vector.
    // To support this behavior, we declare the vector variable as mutable with the mut keyword.
    // Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruit = Vec::new();

    // Push and pop values
    // To add a value to the end of the vector, we use the push(<value>) method.
    // Push values onto end of vector, type changes from generic `T` to String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    /* After the type of a vector is set to a concrete type, only values of that specific type can be added to the vector.
       If we try to add a value of a different type, the compiler returns an error. */

    // To remove the value at the end of the vector, we use the pop() method.
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    // Index into a vector
    // Vectors support indexing in the same manner as arrays.
    // Declare vector, initialize with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    // Add 5 to the value at index 1, which is 5 + 3 = 8
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);
}
