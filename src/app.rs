// filepath: /rust-terminal-emulator/rust-terminal-emulator/src/app.rs

use crate::events;
use crate::ui;

use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub fn run() {
    let host = "host"; // Replace with actual host retrieval logic
    let user = "user"; // Replace with actual user retrieval logic

    println!("Welcome to the App-based Terminal Emulator!");
    println!("Type 'exit' to quit the program");
    println!("-----------------------------------------------------------");

    loop {
        let input = ui::render_prompt_and_get_input(user, host);
        if input.trim() == "exit" {
            println!("Exiting terminal emulator...");
            break;
        }
        
        if let Err(e) = events::capture_input(&input) {
            eprintln!("Error capturing input: {}", e);
        }

        // Simulate some processing delay
        thread::sleep(Duration::from_millis(100));
    }
}