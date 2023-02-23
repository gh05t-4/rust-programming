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