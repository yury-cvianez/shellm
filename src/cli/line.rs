// input and output a line command to the shell

/*

The workflow is as follows: we receive a character typed by the user, 
the input decoder processes the character's byte, determines the state, and accumulates the sequence until an event is formed; 
once the event is formed, the editor—which already holds the buffer and the cursor—receives the event and makes the necessary modifications.

DecoderState: Where do I stand in the interpretation?
InputDecoder: I received a new byte; what is my current state, and what is the next state based on this new byte?
LineEditor: I have the buffer and the cursor position; now I've received an event and need to modify the buffer, the cursor, or both.

*/

use std::io::{stdin};
use std::mem::replace;


enum DecoderState {
    // Defines states used to identify the input event.
    Normal,
    Escape,
    CSI,
}
struct InputDecoder {
    // Decodes the input events from the user
    state: DecoderState,
    utf8_buffer: Vec<u8>,
    utf8_length: usize,
}

impl InputDecoder {
    pub fn new() -> Self {
        InputDecoder {
            state: DecoderState::Normal, 
            utf8_buffer: Vec::new(),
            utf8_length: 0
        }
        
    }

    pub fn process_byte(&mut self, byte: u8) -> Option<InputEvent> {
        match self.state {

            DecoderState::Normal => {
                match byte {
                    b'\x1b' => {
                        self.state = DecoderState::Escape;
                        Some(InputEvent::Character('\x1b'))
                    },
                    b']' =>{
                        self.state = DecoderState::CSI;
                        Some(InputEvent::Character(']'))
                    },
                    _  =>{
                        Some(InputEvent::Character(byte as char))
                    }
                }
            }
            
        }

    }
}

enum InputEvent {
    // Define movements to edit the text
    Character(char),
    ArrowLeft,
    ArrowRight,
    Backspace,
    Delete,
    Enter,
}

struct LineEditor{
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