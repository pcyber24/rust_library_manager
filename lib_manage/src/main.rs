// Add books to the library
// register users
// check out and return booksview current availablility
// store data persistently (e.g., with JSON or SQLite)
// Web interface using frameworks like Actix or Rocket

use std::io::{self, Write};


fn read_input(prompt: &str) -> io::Result<String> {
//     let mut user_input = String::new();
//     io::stdin().read_line(&mut user_input);
//     return user_input;
    print!("{}", prompt);
    io::stdout().flush()?;

}

fn main() {
    let mut users: Vec<String> = Vec::new();
    
    users.push(add_string_to_vector().into());
}
