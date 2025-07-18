// Add books to the library
// register users
// check out and return booksview current availablility
// store data persistently (e.g., with JSON or SQLite)
// Web interface using frameworks like Actix or Rocket

use std::io::{self, Write};

// prompt is a string that is pulled when the funct is called
fn read_input(prompt: &str) -> io::Result<String> {
//     let mut user_input = String::new();
//     io::stdin().read_line(&mut user_input);
//     return user_input;

    // Print the prompt and flush so it actually appears before read_line blocks
    // '?' means ignore I/O errors
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Trim newline/carriage return
    let input = input.trim_end_matches(&['\r', '\n'][..]).to_owned();
    Ok(input)
}

// Reads one line and pushes it into the provided vector
fn add_string_to_vector(users: &mut Vec<String>, prompt: &str) -> io::Result<()>{
    let s = read_input(prompt)?;
    users.push(s);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut users = Vec::new();
    add_string_to_vector(&mut users, "Enter a new name: ")?;
    add_string_to_vector(&mut users, "Enter another: ")?;

    println!("Current users: {:?}", users);
    Ok(())
}
