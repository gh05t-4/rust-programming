/* 
Place a semicolon after mod authentication instead of a code block. As files grow in size, this technique lets you move modules to new files
automatically. The compiler loads the module contents from another file that's named the same as the module.
*/

mod authentication;

fn main() {
    let mut user = authentication::User::new("jeremy", "super-secret");
    println!("User: {:?}", user);
    println!("username: {}", user.get_username());
    println!("password hash: {}", user.get_password_hash());
    user.set_password("password123");
    println!("new password hash: {}", user.get_password_hash());
}
