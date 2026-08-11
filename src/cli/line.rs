// input and output a line command to the shell

use std::io::{stdin};

use crate::cli::line;


enum InputEvent{
    // Define movements to edit the text
    Character(char),
    ArrowLeft,
    ArrowRight,
    Backspace,
    Delete,
    Enter
}
struct LineEditor{
    // Saves the real-time state of what the user types.
    buffer   : Vec<char>,
    cursor   : usize,
}

impl LineEditor {
    
    pub fn new() -> Self {
        // Initialize a new LineEditor with an empty buffer and cursor at position 0
        LineEditor {
            buffer: Vec::new(),
            cursor: 0
        }
    }

    pub fn process_input(&mut self, event: InputEvent) {
        match event {

            InputEvent::Character(c) => {
                // Insert the character at the current cursor position
                self.buffer.insert(self.cursor, c);
                self.cursor += 1;
            }

            InputEvent::ArrowLeft => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }

            InputEvent::ArrowRight => {
                if self.cursor < self.buffer.len() {
                    self.cursor += 1;
                }
            }

            InputEvent::Backspace => {
                if self.cursor > 0 {
                    self.buffer.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
            }


            InputEvent::Delete => {
                if self.cursor < self.buffer.len() {
                    self.buffer.remove(self.cursor);
                }
            }

            InputEvent::Enter => {
                //let line_complete: String = self.buffer.iter().collect::<String>();

                self.buffer.clear();
                self.cursor = 0;
            }
        }

    }

}



pub fn read_line() -> String {
    // read input the user typed in the shell

    let mut input = String::new();
    
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.to_string();
}