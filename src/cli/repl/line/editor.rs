use std::mem::replace;


use crate::cli::repl::line::event::InputEvent;


pub struct LineEditor{
    // Saves the real-time state of what the user types.
    buffer   : Vec<char>,
    cursor   : usize,
}

impl LineEditor {
    
    pub fn new() -> Self {
        LineEditor {
            buffer: Vec::new(),
            cursor: 0,
        }
    }

    pub fn process_input(&mut self, event: InputEvent) -> Option<String>{
        match event {

            InputEvent::Character(c) => {
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
            _ => {
                // temp
                None
            }
        }

    }
    
}