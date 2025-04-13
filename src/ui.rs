// This file handles the user interface components of the terminal emulator.
// It exports functions to render the user and host prompts and display user input.

use std::io::{self, Write, stdin};

pub fn render_prompt(user: &str, host: &str) {
    print!("{}@{}: ", user, host);
    io::stdout().flush().unwrap();
}

// New function that both displays the prompt and gets user input
#[allow(dead_code)]
pub fn render_prompt_and_get_input(user: &str, host: &str) -> String {
    render_prompt(user, host);
    
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    
    // Trim the newline character
    input.trim().to_string()
}

#[allow(dead_code)]
pub fn display_input(input: &str) {
    println!("{}", input);
}