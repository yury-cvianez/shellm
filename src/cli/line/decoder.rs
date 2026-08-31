/* 
A post-keyboard layer that interprets the type of event based on user input by analyzing byte sequences and character formation.
*/

use std::collections::VecDeque;

use crate::cli::line::event::InputEvent;
use crate::cli::line::sequences::{
    Utf8Decoder, Utf8Result,
    CSIDecoder, CSIResult
};

enum DecoderState {
    // Defines states used to identify the input event.
    Normal,
    Escape,
    CSI,
}

struct InputDecoder {
    state   : DecoderState,
    utf8    : Utf8Decoder,
    
    sequence_buffer : Vec<u8>,
    pending_events  : VecDeque<InputEvent>,
}

impl InputDecoder {

    pub fn new() -> Self {

        InputDecoder {
            state   : DecoderState::Normal,
            utf8    : Utf8Decoder::new(),
            
            sequence_buffer : Vec::new(),
            pending_events  : VecDeque::new(),
        }
    }

    pub fn feed_byte(&mut self, byte: u8) {
        // processes an input byte and stores produced events.

        if self.utf8.is_pending(){
            self._process_utf8(byte);
            return;
        }

        self._process_state(byte);
    }

    pub fn next_event(&mut self) -> Option<InputEvent> {
        // returns the next pending input event.

        self.pending_events.pop_front()
    }

    fn _process_utf8(&mut self, byte: u8) {
        // processes a byte belonging to the current sequence.

        match self.utf8.process(byte) {
            
            Utf8Result::Pending => {},

            Utf8Result::Character(c) => {
                self.pending_events.push_back(
                    InputEvent::Character(c)
                );
            },

            Utf8Result::Invalid(bytes) => {
                self.pending_events.push_back(
                    InputEvent::Invalid(bytes)
                );

                self.feed_byte(byte);
            }

        }
    }

    fn _process_state(&mut self, byte: u8) {
        // sends the byte to the current state
        
        match self.state {
            
            DecoderState::Normal => {
                self._handle_normal(byte);
            },
            
            DecoderState::Escape => {
                self._handle_escape(byte);
            },

            DecoderState::CSI => {
                self._handle_csi(byte);
            },

        }
    }

    fn _handle_normal(&mut self, byte: u8) {

        match byte {

            // ESC
            b'\x1b' => {
                self.sequence_buffer.push(byte);
                self.state = DecoderState::Escape;
            },

            // ASCII
            b if b & 0b1000_0000 == 0 => {
                self.pending_events.push_back(
                    InputEvent::Character(byte as char)
                );
            },

            // UTF-8: 2 bytes
            b if b & 0b1110_0000 == 0b1100_0000 => {
                self.utf8.start(byte, 2);
            },

            // UTF-8: 3 bytes
            b if b & 0b1111_0000 == 0b1110_0000 => {
                self.utf8.start(byte, 3);
            },

            // UTF-8: 4 bytes
            b if b & 0b1111_1000 == 0b1111_0000 => {
                self.utf8.start(byte, 4);
            },
            
            // Invalid byte
            _ => {
                self.pending_events.push_back(
                    InputEvent::Invalid(vec![byte])
                );
            },

        }
    }

    fn _handle_escape(&mut self, byte: u8) {

        match byte {

            // Left bracket
            b'\x5b' => {
                self.sequence_buffer.push(byte);
                self.state = DecoderState::CSI;
            },

            // invalid byte for CSI sequence
            _ => {
                self.sequence_buffer.push(byte);
                let invalid = std::mem::take(&mut self.sequence_buffer);
                
                self.pending_events.push_back(
                    InputEvent::Invalid(invalid)
                );
                
                self.state = DecoderState::Normal;
            },

        }
    }

    fn _handle_csi(&mut self, byte: u8) {

    }

}
