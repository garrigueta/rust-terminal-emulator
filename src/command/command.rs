// Command execution module for handling different types of commands
use std::io;
use std::process::Command;
use std::env;
use std::path::{Path, PathBuf};

pub enum CommandResult {
    Output(String),
    Error(String),
    Empty,
    DirectoryChanged(PathBuf),
}

pub trait CommandExecutor {
    fn execute(&self, args: &[&str]) -> io::Result<CommandResult>;
    fn name(&self) -> &str;
    fn help(&self) -> &str;
}

// Bash command executor - runs commands in bash
pub struct BashExecutor;

impl CommandExecutor for BashExecutor {
    fn execute(&self, args: &[&str]) -> io::Result<CommandResult> {
        if args.is_empty() {
            return Ok(CommandResult::Empty);
        }
        
        let command = args.join(" ");
        
        // Special handling for cd command since it affects the process state
        if command.trim().starts_with("cd ") {
            return self.handle_cd_command(&command);
        }
        
        let output = Command::new("bash")
            .arg("-c")
            .arg(&command)
            .output()?;
            
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            return Ok(CommandResult::Error(error));
        }
        
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        if output_str.is_empty() {
            Ok(CommandResult::Empty)
        } else {
            Ok(CommandResult::Output(output_str))
        }
    }
    
    fn name(&self) -> &str {
        "bash"
    }
    
    fn help(&self) -> &str {
        "Executes commands in bash shell"
    }
}

impl BashExecutor {
    // Special handling for cd command
    fn handle_cd_command(&self, command: &str) -> io::Result<CommandResult> {
        // Extract the directory path from the cd command
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        if parts.len() < 2 {
            // Just "cd" without args should go to home directory
            if let Some(home) = dirs::home_dir() {
                env::set_current_dir(&home)?;
                return Ok(CommandResult::DirectoryChanged(home));
            }
            return Ok(CommandResult::Error("Home directory not found".to_string()));
        }
        
        let dir_path = parts[1];
        
        // Handle "cd -" to go to previous directory
        if dir_path == "-" {
            if let Ok(prev_dir) = env::var("OLDPWD") {
                let prev_path = PathBuf::from(prev_dir);
                env::set_current_dir(&prev_path)?;
                
                // Store current dir as OLDPWD for next time
                if let Ok(current) = env::current_dir() {
                    env::set_var("OLDPWD", current.to_string_lossy().to_string());
                }
                
                return Ok(CommandResult::DirectoryChanged(prev_path));
            }
            return Ok(CommandResult::Error("No previous directory".to_string()));
        }
        
        // Save the current directory as OLDPWD
        if let Ok(current) = env::current_dir() {
            env::set_var("OLDPWD", current.to_string_lossy().to_string());
        }
        
        // Handle relative or absolute paths
        let path = PathBuf::from(dir_path);
        
        // Try to change directory
        match env::set_current_dir(&path) {
            Ok(_) => {
                // Get the absolute path after changing
                match env::current_dir() {
                    Ok(new_dir) => Ok(CommandResult::DirectoryChanged(new_dir)),
                    Err(e) => Ok(CommandResult::Error(format!("Failed to get new directory: {}", e))),
                }
            },
            Err(e) => Ok(CommandResult::Error(format!("Failed to change directory: {}", e))),
        }
    }
}

// Command registry to manage available commands
pub struct CommandRegistry {
    executors: Vec<Box<dyn CommandExecutor>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            executors: Vec::new(),
        };
        
        // Register default executors
        registry.register(Box::new(BashExecutor));
        
        registry
    }
    
    pub fn register(&mut self, executor: Box<dyn CommandExecutor>) {
        self.executors.push(executor);
    }
    
    pub fn execute_bash_command(&self, command: &str) -> io::Result<CommandResult> {
        // Find the bash executor
        for executor in &self.executors {
            if executor.name() == "bash" {
                // Create a vector with the command as a single string
                let args = vec![command];
                // Convert to a slice of string references
                let args_slice: Vec<&str> = args.iter().map(|s| &**s).collect();
                return executor.execute(&args_slice);
            }
        }
        
        // If we can't find bash executor, return an error
        Ok(CommandResult::Error("Bash executor not found".to_string()))
    }
}