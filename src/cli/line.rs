// input and output a line command to the shell

use std::io::{stdin};
use std::mem::replace;


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

    pub fn process_input(&mut self, event: InputEvent) -> Option<String>{
        match event {

            InputEvent::Character(c) => {
                // Insert the character at the current cursor position
                self.buffer.insert(self.cursor, c);
                self.cursor += 1;
                None
            }

            InputEvent::ArrowLeft => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                None
            }

            InputEvent::ArrowRight => {
                if self.cursor < self.buffer.len() {
                    self.cursor += 1;
                }
                None
            }

            InputEvent::Backspace => {
                if self.cursor > 0 {
                    self.buffer.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
                None
            }


            InputEvent::Delete => {
                if self.cursor < self.buffer.len() {
                    self.buffer.remove(self.cursor);
                }
                None
            }

            InputEvent::Enter => {
                let line = self.buffer.iter().collect::<String>();
                replace(&mut self.buffer, Vec::new());

                self.cursor = 0;

                Some(line)
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