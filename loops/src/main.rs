fn main() {
    /* 
    Rust offers three loop expressions to make a program repeat a block of code:
        loop: Repeat, unless a manual stop occurs.
        while: Repeat while a condition remains true.
        for: Repeat for all values in a collection.
    */

    // The loop expression creates an infinite loop. This keyword lets us repeat the actions in the expression body continuously.
    // The actions repeat until we take some direct action to make the loop stop.

    /* 
    loop {
        println!("We loop forever!");
    }
    The most common way to stop a loop expression is by using the 'break' keyword to set a break point.
    */

    // The break keyword reveals a special feature of the loop expression.
    // By using the break keyword, you can both stop repeating the actions in the expression body and also return a value at the break point.
    let mut counter = 1;
    // stop_loop is set when loop stops
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("\nBreak the loop at counter = {}.", stop_loop);

    // While Loop
    // The loop repeats as long as the conditional expression remains true.
    counter = 1;
    while counter < 5 {
        println!("We loop a while...");
        counter = counter + 1;
    }

    // For Loop
    // The loop repeats the actions in the expression body for each item in the collection.
    // This type of loop repetition is called iterating.
    // The for loop uses a temporary variable as the iterator.
    // The variable is implicitly declared at the start of the loop expression, and the current value is set with each iteration.
    let big_birds = ["ostrich", "peacock", "stork"];

    // We access the items in the collection by using the iter() method.
    // The for expression binds the current value of the iterator to the result of the iter() method.
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }

    // Another easy way to create an iterator is to use the range notation a..b.
    // The iterator starts at the a value and continues through to b in steps of one, but it doesn't use the value b.
    for number in 0..5 {
        println!("{}", number * 2);
    }    

}
