// filepath: /rust-terminal-emulator/rust-terminal-emulator/src/main.rs

use rust_terminal_emulator::terminal::terminal::Terminal;
use std::{io, process};

fn main() -> io::Result<()> {
    // Create and initialize the terminal
    let mut terminal = Terminal::new()?;
    terminal.init()?;
    
    // Main event loop
    loop {
        // Process keyboard input and check if we should exit
        if !terminal.process_keyboard_input()? {
            break;
        }
    }
    
    // Clean up terminal state
    terminal.cleanup()?;
    
    Ok(())
}