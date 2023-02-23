// STRUCTS
// Classic struct with named fields [a classic struct is defined inside curly brackets {}]
struct Student { name: String, level: u8, remote: bool }

// Tuple struct with data types only [a tuple struct is defined inside parentheses ()]
struct Grades(char, char, char, char, f32);

// ENUMS
// Define a tuple struct
#[derive(Debug)]
struct KeyPress(String, char);

// Define a classic struct
#[derive(Debug)]
struct MouseClick { x: i64, y: i64 }

// Define the WebEvent enum variants to use the data from the structs
// and a boolean type for the page Load variant
#[derive(Debug)]
enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

fn main() {
    // Declare a variable (immutable)
    let a_number;

    // Declare a mutable variable
    let mut x = 10;

    // Declare a second variable and bind the value
    let a_word = "Ten";

    // Bind a value to the variable
    a_number = 10;
    x += 1;

    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);
    println!("Mutable variable x is changed from 10 to {}.", x);

    // Variable shadowing
    // Declare first variable binding with name "shadow_num"
    let shadow_num = 5;

    // Declare second variable binding, shadows existing variable "shadow_num" 
    let shadow_num = shadow_num + 5; 

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2; 

    println!("The shadow number is {}.", shadow_num);

    // Data types: https://learn.microsoft.com/en-us/training/modules/rust-create-program/3-basic-data-types#numbers-integers-and-floating-point-values

    let number_64 = 4.1;    // compiler infers the value to use the default type f64
    let number_32: f32 = 5.6;    // type f32 specified via annotation
    println!("Floating point values: 64-bit={}, 32-bit={}", number_64, number_32);

    // Declare variable to store result of "greater than" test, Is 1 > 4? -- false
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);

    // characters (should be enclosed with single quotation marks)
    let character_1: char = 'S';
    let character_2: char = 'f';
    
    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);

    // TUPLES
    // A tuple has a fixed length, which is equal to its number of elements. After a tuple is declared, it can't grow or shrink in size. Elements can't be added or removed.

    // Tuple of length 3
    let tuple_e = ('E', 5i32, true);
    // Use tuple indexing and show the values of the elements in the tuple
    print!("\nTuple: ");
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);

    // STRUCTS
    
    // Instantiate classic struct, specify fields in random order, or in specified order
    // *NOTE*: String data that's stored inside another data structure, such as a struct or vector,
    // must be converted from a string literal reference (&str) to a String type. To do the conversion,
    // we use the standard String::from(&str) method.
    let user_1 = Student { name: String::from("Sanjay"), remote: true, level: 2 };
    let user_2 = Student { name: String::from("Drupad"), level: 5, remote: false };

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);
    println!();
    println!("Structs:");

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);

    // ENUMS (https://learn.microsoft.com/en-us/training/modules/rust-create-program/5-enum-variants)
    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };
    println!("\nEnums:\nMouse click location: {}, {}", click.x, click.y);
        
    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);
        
    // Instantiate WebEvent enum variants
    // Set the boolean page Load value to true
    let we_load = WebEvent::WELoad(true);
    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);
        
    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);

}
