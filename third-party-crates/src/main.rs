/* 
Rust standard library doesn't have a module for regular expressions, so let's add the regex crate that's available at https://crates.io.
This website is the Rust community's central package registry that serves as a location to discover and download packages.

Whenever we want to add dependent crates to our project, we can rely on Cargo to do all the heavy lifting for us.
To depend on a library hosted on crates.io, add it to your Cargo.toml file:

[dependencies]
regex = "1.4.2"

If your Cargo.toml doesn't already have a [dependencies] section, add that section. Then list the crate name and version that you want to use.

The next step is to run the command 'cargo build'. Cargo will fetch the new dependency and all of its dependencies, and compile them all
*/

// We can now use the regex library in our main.rs

use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2018-02-01"));
}
