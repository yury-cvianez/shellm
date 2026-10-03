use std::io::Write;
use std::mem::replace;


use crate::cli::repl::line::event::InputEvent;
use crate::completion::completer::{Completer};



const PROMPT: &str = "$ ";

pub struct LineEditor<C: Completer>{
    // Saves the real-time state of what the user types.
    buffer   : Vec<char>,
    cursor   : usize,
    completer: C,
    completions: Vec<(String, String)>,
}

impl<C: Completer> LineEditor<C> {

    pub fn new(completer: C) -> Self {

        LineEditor {
            buffer: Vec::new(),
            cursor: 0,
            completer,
            completions: Vec::new(),
        }
    }

    pub fn process_input(&mut self, event: InputEvent) -> Option<String>{
        
        //println!("Buffer: {:?}, Cursor: {}", self.buffer, self.cursor);
        
        let result = match event {

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
        };

        match result {
            Some(_) => self.clear_line(), // a moldura vai ocupar o lugar da linha
            None => self.render(),
        }
        result

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
        
        let completed = self.buffer.iter().collect::<String>();
        self.completions.push((prefix, completed)); 
        
    }

    pub fn take_completions(&mut self) -> Vec<(String, String)> {
        std::mem::take(&mut self.completions)
    }
        
    pub fn render(&self) {
        let text: String = self.buffer.iter().collect();
        let col = PROMPT.len() + self.cursor + 1; // \x1b[nG é 1-based
        let mut out = std::io::stdout().lock();
        let _ = write!(out, "\r\x1b[2K{PROMPT}{text}\x1b[{col}G");
        let _ = out.flush(); // sem \n o stdout não descarrega sozinho
    }

    fn clear_line(&self) {
        let mut out = std::io::stdout().lock();
        let _ = write!(out, "\r\x1b[2K");
        let _ = out.flush();
    }

}