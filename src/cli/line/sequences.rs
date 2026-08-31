/* 
responsible for the decoding sequences
*/

pub struct Utf8Decoder {
    buffer: Vec<u8>,
    length: usize,
}

pub enum Utf8Result {
    Pending,
    Character(char),
    Invalid(Vec<u8>),
}

impl Utf8Decoder {

    pub fn new() -> Self {
        // initial state the character
        Utf8Decoder {
            buffer : Vec::new(),
            length : 0
        }
    }

    pub fn is_pending(&self) -> bool {
        // if true, we are already within a decoding sequence.

        self.length > 0 && self.buffer.len() < self.length
    }
    
    pub fn start(&mut self, fbyte: u8, length: usize) {

        self._reset();

        self.length = length;
        self.buffer.push(fbyte);
    }

    pub fn process(&mut self, byte: u8) -> Utf8Result {
        // Continues assembling the character.
        
        if byte & 0b1100_0000 != 0b1000_0000 {
            // Invalid UTF-8 reset the sequence
            
            self.buffer.push(byte);
            let invalid = std::mem::take(&mut self.buffer);
            
            self._reset();

            return Utf8Result::Invalid(invalid)
        }

        self.buffer.push(byte);
        
        if self.buffer.len() < self.length { 
            return Utf8Result::Pending; 
        }

        self._finish()
    }

    pub fn _finish(&mut self) -> Utf8Result {

        let bytes = std::mem::take(&mut self.buffer);

        self._reset();

        match std::str::from_utf8(&bytes) {

            Ok(s) => {
                let character = s.chars().next().unwrap();
                Utf8Result::Character(character)
            }

            Err(_) => {
                Utf8Result::Invalid(bytes)
            }
        }
    }

    fn _reset(&mut self) {

        self.buffer.clear();
        self.length = 0;
    }

}
