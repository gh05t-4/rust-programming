fn main() {
    // The HashMap<K, V> type stores data by mapping each key K with its value V.
    // While data in a vector is accessed by using an integer index, data in a hash map is accessed by using a key.


    // Define a hash map

    // The use command brings the HashMap definition from the collections portion of the Rust standard library into scope for our program.
    // This syntax is similar to what other programming languages call an import.
    use std::collections::HashMap;

    /* 
    We create an empty hash map with the HashMap::new method.
    We declare the reviews variable as mutable so we can add or remove keys and values, as needed.
    In our example, both the hash map keys and values use the String type.
    */
    let mut reviews: HashMap<String, String> = HashMap::new();

    // Add a key-value pair
    // We add elements to the hash map by using the insert(<key>, <value>) method. In the code, the syntax is <hash_map_name>.insert()
    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

    // Get a key value
    // We can get a specific value for a key with the get(<key>) method.
    // Look for a specific review
    let book = "Programming in Rust";
    println!("\nReview for '{}': {:?}", book, reviews.get(book));
    // NOTE: Notice the output displays the book review as "Some("Great examples.")" rather than just "Great examples."
    // Because the get method returns an Option<&Value> type, Rust wraps the result of the method call with the "Some()" notation.

    // Remove a key-value pair
    // We can remove entries from a hash map by using the .remove() method.
    // If we use the get method for an invalid hash map key, the get method returns "None."
    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));

}
