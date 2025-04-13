// Terminal module that encapsulates terminal functionality
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, 
        LeaveAlternateScreen, size
    },
};
use std::{
    io::{self, Write},
    time::Duration,
    env, // Add this for current directory functions
    path::PathBuf, // Add this for path manipulation
};

use crate::command::command::{CommandRegistry, CommandResult};

// Store the terminal state
pub struct Terminal {
    pub width: u16,
    pub height: u16,
    pub history: Vec<String>,
    pub scroll_position: usize,
    pub prompt: String,
    pub input_buffer: String,
    pub command_history: Vec<String>,
    pub command_history_position: Option<usize>,
    pub command_registry: CommandRegistry,
    pub current_dir: PathBuf, // Add current directory tracking
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        // Get terminal size
        let (width, height) = size()?;
        
        Ok(Self {
            width,
            height,
            history: Vec::new(),
            scroll_position: 0,
            prompt: String::from("user@host: "),
            input_buffer: String::new(),
            command_history: Vec::new(),
            command_history_position: None,
            command_registry: CommandRegistry::new(),
            current_dir: env::current_dir()?, // Initialize current directory
        })
    }

    pub fn init(&mut self) -> io::Result<()> {
        // Enable raw mode and enter alternate screen
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        
        // Add Rust ASCII logo
        self.add_to_history("".to_string());
        self.add_to_history("                 _~^~^~_                 ".to_string());
        self.add_to_history("             \\) /  o o  \\ (/            ".to_string());
        self.add_to_history("               '_   v   _'               ".to_string());
        self.add_to_history("              / '-----' \\               ".to_string());
        self.add_to_history("                                         ".to_string());
        self.add_to_history("         ██████╗ ██╗   ██╗███████╗████████╗".to_string());
        self.add_to_history("         ██╔══██╗██║   ██║██╔════╝╚══██╔══╝".to_string());
        self.add_to_history("         ██████╔╝██║   ██║███████╗   ██║   ".to_string());
        self.add_to_history("         ██╔══██╗██║   ██║╚════██║   ██║   ".to_string());
        self.add_to_history("         ██║  ██║╚██████╔╝███████║   ██║   ".to_string());
        self.add_to_history("         ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ".to_string());
        self.add_to_history("                                         ".to_string());
        self.add_to_history("  ******** Rust Terminal Emulator ******** ".to_string());
        self.add_to_history("".to_string());
        self.add_to_history("Use Ctrl+Up/Down or PageUp/PageDown to scroll through terminal history.".to_string());
        self.add_to_history("Type 'exit' or press ESC to quit.".to_string());
        self.add_to_history("".to_string());
        
        // Render initial screen
        self.render()?;
        
        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        // Disable raw mode and leave alternate screen
        execute!(io::stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn add_to_history(&mut self, line: String) {
        self.history.push(line);
        // Auto-scroll to the bottom when adding new content
        self.scroll_to_bottom();
    }

    pub fn scroll_up(&mut self, lines: usize) {
        if self.scroll_position > 0 {
            self.scroll_position = self.scroll_position.saturating_sub(lines);
        }
    }

    pub fn scroll_down(&mut self, lines: usize) {
        let max_scroll = self.max_scroll();
        if self.scroll_position < max_scroll {
            self.scroll_position = (self.scroll_position + lines).min(max_scroll);
        }
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_position = self.max_scroll();
    }

    fn max_scroll(&self) -> usize {
        self.history.len().saturating_sub(self.height as usize - 1)
    }

    pub fn render(&self) -> io::Result<()> {
        // Clear the screen
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        
        // Calculate visible range
        let visible_end = (self.scroll_position + self.height as usize - 1).min(self.history.len());
        let visible_range = self.scroll_position..visible_end;
        
        // Render visible history
        for (i, line) in self.history[visible_range].iter().enumerate() {
            // Position cursor and print line
            execute!(io::stdout(), MoveTo(0, i as u16), Print(line))?;
        }
        
        // Get dynamic prompt with actual path
        let dynamic_prompt = self.get_prompt();
        
        // Position cursor for input line at the bottom
        execute!(
            io::stdout(), 
            MoveTo(0, self.height - 1),
            SetForegroundColor(Color::Green),
            Print(&dynamic_prompt),
            ResetColor,
            Print(&self.input_buffer)
        )?;
        
        io::stdout().flush()?;
        
        Ok(())
    }

    pub fn process_keyboard_input(&mut self) -> io::Result<bool> {
        // Check for keyboard events with a timeout
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match code {
                    KeyCode::Esc => {
                        self.add_to_history("Exiting...".to_string());
                        self.render()?;
                        return Ok(false); // Signal to exit
                    }
                    
                    KeyCode::Enter => {
                        // Process the current input
                        let command = self.input_buffer.clone();
                        
                        // Get the current dynamic prompt
                        let current_prompt = self.get_prompt();
                        
                        // Add command to history display with the current dynamic prompt
                        self.add_to_history(format!("{}{}", current_prompt, command));
                        
                        // Check for exit command
                        if command.trim() == "exit" {
                            self.add_to_history("Exiting...".to_string());
                            self.render()?;
                            return Ok(false); // Signal to exit
                        }
                        
                        // Add to command history if not empty
                        if !command.trim().is_empty() {
                            self.command_history.push(command.clone());
                            self.command_history_position = None;
                        }
                        
                        // Clear input buffer
                        self.input_buffer.clear();
                        
                        // Process and display command output
                        if !command.trim().is_empty() {
                            self.execute_command(&command)?;
                        }
                        
                        // Re-render after command execution
                        self.render()?;
                    }
                    
                    KeyCode::Char(c) => {
                        // Add character to input buffer
                        self.input_buffer.push(c);
                        self.render()?;
                    }
                    
                    KeyCode::Backspace => {
                        // Remove last character from input
                        if !self.input_buffer.is_empty() {
                            self.input_buffer.pop();
                            self.render()?;
                        }
                    }
                    
                    KeyCode::Up => {
                        if modifiers.contains(KeyModifiers::CONTROL) {
                            // Scroll up with Ctrl+Up
                            self.scroll_up(1);
                            self.render()?;
                        } else {
                            // Navigate command history (up)
                            self.navigate_history_up()?;
                        }
                    }
                    
                    KeyCode::Down => {
                        if modifiers.contains(KeyModifiers::CONTROL) {
                            // Scroll down with Ctrl+Down
                            self.scroll_down(1);
                            self.render()?;
                        } else {
                            // Navigate command history (down)
                            self.navigate_history_down()?;
                        }
                    }
                    
                    KeyCode::PageUp => {
                        // Scroll up one page
                        self.scroll_up(self.height as usize / 2);
                        self.render()?;
                    }
                    
                    KeyCode::PageDown => {
                        // Scroll down one page
                        self.scroll_down(self.height as usize / 2);
                        self.render()?;
                    }
                    
                    _ => {}
                }
            }
        }
        
        Ok(true) // Continue running
    }
    
    fn navigate_history_up(&mut self) -> io::Result<()> {
        if self.command_history.is_empty() {
            return Ok(());
        }
        
        let new_pos = match self.command_history_position {
            Some(pos) if pos > 0 => Some(pos - 1),
            None => Some(self.command_history.len() - 1),
            other => other,
        };
        
        if let Some(pos) = new_pos {
            self.input_buffer = self.command_history[pos].clone();
            self.command_history_position = Some(pos);
            self.render()?;
        }
        
        Ok(())
    }
    
    fn navigate_history_down(&mut self) -> io::Result<()> {
        if self.command_history.is_empty() {
            return Ok(());
        }
        
        let new_pos = match self.command_history_position {
            Some(pos) if pos < self.command_history.len() - 1 => Some(pos + 1),
            Some(_) => None, // At the end, clear the buffer
            None => None,
        };
        
        match new_pos {
            Some(pos) => {
                self.input_buffer = self.command_history[pos].clone();
                self.command_history_position = Some(pos);
            },
            None => {
                self.input_buffer.clear();
                self.command_history_position = None;
            }
        }
        
        self.render()?;
        Ok(())
    }

    // Execute a command using the command registry
    fn execute_command(&mut self, command: &str) -> io::Result<()> {
        // Temporarily disable raw mode while running the command
        disable_raw_mode()?;
        
        // Execute the command using our command registry
        match self.command_registry.execute_bash_command(command) {
            Ok(result) => {
                match result {
                    CommandResult::Output(output) => {
                        // Split output by lines and add each to history
                        for line in output.lines() {
                            self.add_to_history(line.to_string());
                        }
                    },
                    CommandResult::Error(error) => {
                        // Display error with a prefix
                        for line in error.lines() {
                            self.add_to_history(format!("Error: {}", line));
                        }
                    },
                    CommandResult::Empty => {
                        // Command executed successfully but produced no output
                    },
                    CommandResult::DirectoryChanged(new_dir) => {
                        // Update our tracked current directory
                        self.current_dir = new_dir;
                    }
                }
            },
            Err(e) => {
                self.add_to_history(format!("Failed to execute command: {}", e));
            }
        }
        
        // Re-enable raw mode
        enable_raw_mode()?;
        
        Ok(())
    }

    // Update current directory
    pub fn update_current_dir(&mut self, new_dir: PathBuf) -> io::Result<()> {
        // Attempt to change to the new directory
        env::set_current_dir(&new_dir)?;
        self.current_dir = new_dir;
        Ok(())
    }
    
    // Generate a prompt with the actual path
    pub fn get_prompt(&self) -> String {
        // Get current user
        let username = env::var("USER").unwrap_or_else(|_| "user".to_string());
        
        // Get hostname
        let hostname = hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "host".to_string());
        
        // Format the prompt with current directory
        format!("{}@{}({}): ", username, hostname, self.current_dir.display())
    }
    
    // Update the terminal after a command may have changed directory
    pub fn update_after_command(&mut self) -> io::Result<()> {
        // Check if current directory changed
        let new_dir = env::current_dir()?;
        if new_dir != self.current_dir {
            self.current_dir = new_dir;
        }
        Ok(())
    }
}