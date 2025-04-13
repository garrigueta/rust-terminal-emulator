# Rust Terminal Emulator

~~A modern terminal emulator written in Rust with an elegant, modular architecture and interactive features...~~

That's not true, this is a simple Vibe-coding project, never trust an AI 

![Rust Terminal Emulator](https://raw.githubusercontent.com/rust-lang/rust-artwork/master/logo/rust-logo-128x128.png)

## Features

- **Interactive Command Execution**: Run bash commands with proper output capture
- **Directory Navigation**: Full support for directory changes with `cd` command
- **Dynamic Path Display**: Shows your actual path in the prompt (`username@hostname(/current/path):`)
- **Command History**: Navigate through previous commands with Up/Down arrow keys
- **Scrollable Output**: Scroll through terminal history with Ctrl+Up/Down or PageUp/PageDown
- **Visual Feedback**: Colorized prompts and ASCII art welcome screen
- **Clean Exit**: Exit the terminal by typing `exit` or pressing ESC

## Project Structure

```
rust-terminal-emulator
├── src
│   ├── lib.rs               # Module declarations
│   ├── main.rs              # Application entry point
│   ├── command/             # Command execution modules
│   │   ├── mod.rs           # Command module declarations
│   │   └── command.rs       # Command execution logic
│   └── terminal/            # Terminal handling modules
│       ├── mod.rs           # Terminal module declarations
│       └── terminal.rs      # Terminal display and interaction logic
├── Cargo.toml               # Project dependencies
└── README.md                # Project documentation
```

## Dependencies

- **crossterm**: Terminal handling and user interface
- **hostname**: System hostname detection
- **dirs**: Directory path handling

## Setup Instructions

1. Ensure you have Rust installed on your machine:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```
   git clone <repository-url>
   ```

3. Navigate to the project directory:
   ```
   cd rust-terminal-emulator
   ```

4. Build the project:
   ```
   cargo build
   ```

## Usage

Run the terminal emulator:

```
cargo run
```

### Key Commands

- **Navigation**: 
  - `cd <directory>` - Change directory
  - `cd ..` - Move up one directory
  - `cd` - Go to home directory
  - `cd -` - Go to previous directory

- **Terminal Control**:
  - Type `exit` or press `ESC` to exit
  - `Ctrl+Up/Down` - Scroll terminal history
  - `PageUp/PageDown` - Scroll one page at a time
  - `Up/Down` arrows - Navigate command history

## Future Enhancements

- **Syntax Highlighting**: Colorized output for different command types
- **Tab Completion**: Auto-completion for commands and file paths
- **Multiple Tabs/Panes**: Split view or tabbed interface
- **Themes**: Customizable colors and appearance
- **Plugin System**: Extensible functionality via plugins
- **Search**: Find text in terminal history

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- Rust community for excellent libraries
- Crossterm for terminal manipulation functionality
