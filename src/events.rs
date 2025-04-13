// This file defines the functionality for capturing keyboard events. 
// It exports functions to listen for and process keyboard input.

use crossterm::event::{self, KeyEvent, KeyCode, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, size};
use crossterm::cursor::{MoveToColumn, MoveTo};
use crossterm::execute;
use std::io::{self, Write};
use std::process::Command;
use crate::ui;

// Define a struct to hold terminal output and scrolling state
struct TerminalState {
    output_lines: Vec<String>,
    scroll_position: usize,
    terminal_height: u16,
}

impl TerminalState {
    fn new() -> Self {
        let (_, terminal_height) = size().unwrap_or((80, 24));
        Self {
            output_lines: Vec::new(),
            scroll_position: 0,
            terminal_height: terminal_height,
        }
    }

    fn add_output_line(&mut self, line: String) {
        self.output_lines.push(line);
        // Auto-scroll to bottom when new content is added
        self.scroll_position = self.output_lines.len().saturating_sub(self.terminal_height as usize);
    }

    fn add_command_output(&mut self, output: &[u8]) {
        if output.is_empty() {
            return;
        }
        
        let output_str = String::from_utf8_lossy(output);
        
        // Handle the case where output has no newlines
        if !output_str.contains('\n') {
            self.add_output_line(output_str.trim_end().to_string());
            return;
        }
        
        // Split by newlines and handle each line properly
        for line in output_str.lines() {
            // Skip empty lines at the end
            if line.is_empty() && output_str.ends_with('\n') {
                continue;
            }
            self.add_output_line(line.to_string());
        }
    }

    fn scroll_up(&mut self, lines: usize) {
        if self.scroll_position > 0 {
            self.scroll_position = self.scroll_position.saturating_sub(lines);
        }
    }

    fn scroll_down(&mut self, lines: usize) {
        let max_scroll = self.output_lines.len().saturating_sub(self.terminal_height as usize);
        if self.scroll_position < max_scroll {
            self.scroll_position = (self.scroll_position + lines).min(max_scroll);
        }
    }

    fn render_visible_content(&self) -> io::Result<()> {
        // Clear screen
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        
        // Calculate the range of lines to display
        let visible_end = self.output_lines.len().min(self.scroll_position + (self.terminal_height as usize).saturating_sub(1));
        
        // Display the scrollable content one line at a time
        for line in self.output_lines.iter().skip(self.scroll_position).take(visible_end.saturating_sub(self.scroll_position)) {
            // Print exactly one line and move to the beginning of the next line
            println!("{}", line);
        }
        
        Ok(())
    }
}

// Function referenced in app.rs
#[allow(dead_code)]
pub fn capture_input(input: &str) -> io::Result<()> {
    if input.trim().is_empty() {
        return Ok(());
    }
    
    // Run the user input as a bash command
    let output = Command::new("bash")
        .arg("-c")
        .arg(input)
        .output()?;
    
    // Print the command output
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    io::stdout().flush()?;
    
    Ok(())
}

pub fn capture_keyboard_events() {
    // Enable raw mode
    enable_raw_mode().expect("Failed to enable raw mode");

    let mut input_buffer = String::new();
    let mut command_history: Vec<String> = Vec::new();
    let mut terminal_state = TerminalState::new();
    
    // Clear the screen before starting
    execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    
    // Add a welcome message to the terminal state
    terminal_state.add_output_line("Welcome to Rust Terminal Emulator!".to_string());
    terminal_state.add_output_line("Type commands to execute them, use Ctrl+Up/Down or PageUp/PageDown to scroll history.".to_string());
    terminal_state.add_output_line("Type 'exit' or press ESC to quit.".to_string());
    terminal_state.add_output_line("".to_string());
    
    // Render the initial screen content
    terminal_state.render_visible_content().unwrap();
    
    // Display the initial prompt
    display_prompt();

    loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let event::Event::Key(KeyEvent { code, modifiers, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Esc => {
                        terminal_state.add_output_line("\nExiting...".to_string());
                        terminal_state.render_visible_content().unwrap();
                        break;
                    }
                    KeyCode::Enter => {
                        // Clear the current line before processing the command
                        execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).unwrap();
                        
                        // Store the command with its prompt
                        let full_command_line = format!("{}@{}: {}", "user", "host", input_buffer);
                        terminal_state.add_output_line(full_command_line);
                        
                        // Check for exit command
                        if input_buffer.trim() == "exit" {
                            terminal_state.add_output_line("Exiting terminal emulator...".to_string());
                            terminal_state.render_visible_content().unwrap();
                            break;
                        }

                        // Add the command to the history
                        if !input_buffer.trim().is_empty() {
                            command_history.push(input_buffer.clone());
                        }

                        // Make a copy of the input buffer
                        let command = input_buffer.clone();
                        
                        // Clear the input buffer before executing the command
                        input_buffer.clear();

                        // Disable raw mode before running command to ensure proper output
                        disable_raw_mode().expect("Failed to disable raw mode");
                        
                        // Run the user input as a bash command
                        let output = Command::new("bash")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Failed to execute command");

                        // Use the add_command_output method to process command output
                        terminal_state.add_command_output(&output.stdout);
                        terminal_state.add_command_output(&output.stderr);
                        
                        // Re-enable raw mode
                        enable_raw_mode().expect("Failed to re-enable raw mode");

                        // Render the updated terminal content
                        terminal_state.render_visible_content().unwrap();
                        
                        // Display the prompt again after command execution
                        display_prompt();
                    }
                    KeyCode::Char(c) => {
                        // Append the character to the input buffer
                        input_buffer.push(c);

                        // Clear the current line and re-render the input buffer with prompt
                        execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).unwrap();
                        display_prompt_with_input(&input_buffer);
                        io::stdout().flush().unwrap();
                    }
                    KeyCode::Backspace => {
                        // Handle backspace: remove the last character from the buffer
                        if !input_buffer.is_empty() {
                            input_buffer.pop();

                            // Clear the current line and re-render the input buffer with prompt
                            execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).unwrap();
                            display_prompt_with_input(&input_buffer);
                            io::stdout().flush().unwrap();
                        }
                    }
                    KeyCode::Up => {
                        if modifiers.contains(KeyModifiers::CONTROL) {
                            // Scroll up when Ctrl+Up is pressed
                            terminal_state.scroll_up(1);
                            terminal_state.render_visible_content().unwrap();
                            display_prompt_with_input(&input_buffer);
                        } else {
                            // Show previous command from history
                            if !command_history.is_empty() {
                                // Simple history navigation (last command)
                                let last_command = command_history.last().unwrap_or(&String::new()).clone();
                                input_buffer = last_command;

                                // Clear the current line and re-render the input buffer with prompt
                                execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).unwrap();
                                display_prompt_with_input(&input_buffer);
                                io::stdout().flush().unwrap();
                            }
                        }
                    }
                    KeyCode::Down => {
                        if modifiers.contains(KeyModifiers::CONTROL) {
                            // Scroll down when Ctrl+Down is pressed
                            terminal_state.scroll_down(1);
                            terminal_state.render_visible_content().unwrap();
                            display_prompt_with_input(&input_buffer);
                        }
                    }
                    KeyCode::PageUp => {
                        // Scroll up a page
                        terminal_state.scroll_up(terminal_state.terminal_height as usize / 2);
                        terminal_state.render_visible_content().unwrap();
                        display_prompt_with_input(&input_buffer);
                    }
                    KeyCode::PageDown => {
                        // Scroll down a page
                        terminal_state.scroll_down(terminal_state.terminal_height as usize / 2);
                        terminal_state.render_visible_content().unwrap();
                        display_prompt_with_input(&input_buffer);
                    }
                    _ => {}
                }
            }
        }
    }

    // Disable raw mode before exiting
    disable_raw_mode().expect("Failed to disable raw mode");
}

// Helper function to display the prompt
fn display_prompt() {
    ui::render_prompt("user", "host");
}

// Helper function to display the prompt with the current input
fn display_prompt_with_input(input: &str) {
    ui::render_prompt("user", "host");
    print!("{}", input);
}