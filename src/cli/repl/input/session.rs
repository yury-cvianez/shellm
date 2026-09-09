use crate::cli::repl::input::termguard;

use std::os::unix::io::AsRawFd;
use crate::cli::repl::line::decoder::InputDecoder;
use crate::cli::repl::line::editor::LineEditor;

pub struct Input {
    fd      : i32,
    _guard  : termguard::TermiosGuard, 
    
    editor  : LineEditor,
    decoder : InputDecoder,
}

impl Input {
    pub fn new() -> std::io::Result<Self> {
        let fd: i32 = std::io::stdin().as_raw_fd();
        let guard =  termguard::TermiosGuard::new(fd)?;
        guard.enable_raw_mode()?;
        
        Ok(Input { 
            fd,
            _guard: guard, 

            decoder : InputDecoder::new(), 
            editor  : LineEditor::new(),
        })
    }

    pub fn next_line(&mut self) -> std::io::Result<Option<String>> {

        let mut buffer = [0u8; 1];
        
        loop {
            let n = unsafe {
                libc::read(
                    self.fd,
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
            
            if let Some(line) = self._entry_byte(byte) {
                return Ok(Some(line));
            }
        };

        Ok(None)
    }

    fn _entry_byte(&mut self, byte: u8) -> Option<String>{

        self.decoder.feed_byte(byte);

        while let Some(event) = self.decoder.next_event() {

            if let Some(line) = self.editor.process_input(event) {
                return Some(line);
            }
        }

        None
    }
}
