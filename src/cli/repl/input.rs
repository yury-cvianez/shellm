pub mod termguard;

use std::os::unix::io::AsRawFd;
use crate::cli::repl::line::decoder::InputDecoder;
use crate::cli::repl::line::editor::LineEditor;

struct Input {
    decoder : InputDecoder,
    editor  : LineEditor,
}

impl Input {
    pub fn new() -> Self {
        
        Input { 
            decoder: InputDecoder::new(), 
            editor: LineEditor::new() 
        }
    }

    pub fn entry_byte(&mut self, byte: u8) -> std::io::Result<Option<String>> {

        self.decoder.feed_byte(byte);

        while let Some(event) = self.decoder.next_event() {
            println!("Event: {:?}", event);

            if let Some(line) = self.editor.process_input(event) {
                println!("Line entered: {}", line);
                return Ok(Some(line))
            }
        }

        Ok(None)

    }
}

pub fn session() -> std::io::Result<()> {
    
    let fd = std::io::stdin().as_raw_fd();
    let _guard = termguard::TermiosGuard::new(fd).unwrap();
    _guard.enable_raw_mode().unwrap();
    
    let input = &mut Input::new();

    let mut buffer = [0u8; 1];
    
    loop {
        let n = unsafe {
            libc::read(
                fd,
                buffer.as_mut_ptr() as *mut libc::c_void,
                1
            )   
        };
        
        if n < 0 {
            return Err(std::io::Error::last_os_error());
        }
        
        if n == 0 {
            break;
        }
        
        let byte = buffer[0];
        println!("Byte read: {}", byte);
        
        let _ = input.entry_byte(byte);
    };

    Ok(())

}