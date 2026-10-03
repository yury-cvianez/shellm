use std::mem::replace;


use crate::cli::repl::line::event::InputEvent;
use crate::completion::completer::{Completer};


pub struct LineEditor<C: Completer>{
    // Saves the real-time state of what the user types.
    buffer   : Vec<char>,
    cursor   : usize,
    completer: C,
}

impl<C: Completer> LineEditor<C> {

    pub fn new(completer: C) -> Self {

        LineEditor {
            buffer: Vec::new(),
            cursor: 0,
            completer,
        }
    }

    pub fn process_input(&mut self, event: InputEvent) -> Option<String>{
        
        println!("Buffer: {:?}, Cursor: {}", self.buffer, self.cursor);
        
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

            // InputEvent::Delete => {
            //     if self.cursor < self.buffer.len() {
            //         self.buffer.remove(self.cursor);
            //     }
            //     None
            // }

            InputEvent::Enter => {
                let line = self.buffer.iter().collect::<String>();
                replace(&mut self.buffer, Vec::new());

                self.cursor = 0;

                Some(line)
            }
            
            // call autocompletion
            InputEvent::Tab => {
                self.complete();
                None
            }
            
            _ => {
                // temp
                None
            }
        }

    }

    fn complete(&mut self) {

        let prefix = self.buffer.iter().collect::<String>();

        let Ok(completions) = self.completer.complete(&prefix) else {
            return;
        };

        let Some(completion) = completions.first() else {
            return;
        };

        self.buffer.extend(completion.text.chars());

        self.cursor = self.buffer.len();

        println!(
            "COMPLETION: {:?}, Cursor: {}",
            self.buffer,
            self.cursor
        );
        
    }
        
}