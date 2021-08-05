//! A helper function for displaying prompts in the terminal.

use std::io::{
    self,
    Write
};

/// Helper function that correctly renders single-line prompts in the terminal.
pub fn display_prompt(prompt: String) {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
}
