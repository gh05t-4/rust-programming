/* 

   - A package:
       - Contains functionality within one or more crates.
       - Includes information about how to build those crates. The information is in the Cargo.toml file.
   - A crate:
       - Is a compilation unit, which is the smallest amount of code that the Rust compiler can operate on.
       - Once compiled, produces either an executable or a library.
       - Contains an implicit, unnamed top-level module.
   - A module:
       - Is a (possibly nested) unit of code organization inside a crate.
       - Can have recursive definitions that span additional modules.

*/


// Package
/* 
Whenever we run the command $ cargo new <project-name>, Cargo creates a package.
$ cargo new my-project
     Created binary (application) `my-project` package

Here, we have a package that only contains src/main.rs, which means it only contains a binary crate named my-project:

my-project
├── src
│  └── main.rs
└── Cargo.toml

A package can have multiple binary crates by placing files in the src/bin directory. Each file will be a separate binary crate.

If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary. They both have the same name as the package.
*/


// Crates
/* 
Rust's compilation model centers on artifacts called crates that can be compiled into a binary or into a library.

Every project that you create with the cargo new command is a crate itself. All third-party Rust code that you can use as dependencies in your project is
also, each, a single crate. 

Library crates

We've already covered how to create a binary program, and creating a library is just as easy. To create a library, pass the
--lib command-line parameter to the cargo new command:
$ cargo new --lib my-library
     Created library `my-library` package

You can see that instead of a src/main.rs file, you now get a =src/lib.rs' file.

my-library
├── src
│  └── lib.rs
└── Cargo.toml

When you tell Cargo to compile this crate, you'll get a library file called libmy_library.rlib that can be published and linked to other projects.
*/


// Modules
/* 
Rust provides a powerful module system that can be used to hierarchically split code into logical units that also ease readability and reuse.

Modules also control item privacy. Item privacy identifies an item as either public or private. Public means that the item can be used by outside
code. Private means the item is an internal implementation detail and not available for outside use.

If a source file has 'mod' declarations in it, the contents of the module files would be inserted in places where mod declarations in the source
file are found, before running the compiler over it. In other words, modules don't get compiled individually, only crates get compiled.
*/

// simplified authentication API
mod authentication {
    use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

    #[derive(Debug)]
    pub struct User {
        username: String, // private
        password_hash: u64 // private
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_string(),
                password_hash: hash_password(password),
            }
        }

        // If we wanted to give read access to the username field and read/write access to the password field while keeping them private,
        // we could use getter and setter methods
        pub fn get_username(&self) -> &String {
            &self.username
        }

        pub fn get_password_hash(&self) -> &u64 {
            &self.password_hash
        }

        pub fn set_password(&mut self, new_password: &str) {
            self.password_hash = hash_password(new_password)
        }
    }

    fn hash_password(input: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        hasher.finish()
    }
}

fn main() {
    let mut user = authentication::User::new("jeremy", "super-secret");
    println!("User: {:?}", user);
    println!("username: {}", user.get_username());
    println!("password hash: {}", user.get_password_hash());
    user.set_password("password123");
    println!("new password hash: {}", user.get_password_hash());
}
