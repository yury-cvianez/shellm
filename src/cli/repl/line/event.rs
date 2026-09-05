/* 
represents what the decoder translates and what the editor can do
*/

#[derive(Debug)]
pub enum InputEvent {
    // Define movements to edit the text
    Character(char),
    Invalid(Vec<u8>),
    
    ArrowLeft,
    ArrowRight,
    
    Backspace,
    Delete,
    Enter,
}
