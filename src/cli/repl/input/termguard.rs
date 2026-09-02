use libc::{tcgetattr, tcsetattr, termios, TCSANOW};

pub struct TermiosGuard {
    fd: i32,
    original_termios: termios,
}

impl TermiosGuard {
    pub fn new(fd: i32) -> Result<Self, std::io::Error> {

        let mut original_termios = unsafe { std::mem::zeroed() };

        if unsafe { tcgetattr(fd, &mut original_termios) } != 0 {
            return Err(std::io::Error::last_os_error());
        }        

        Ok(TermiosGuard{fd, original_termios})
    }
}

impl Drop for TermiosGuard {
    fn drop(&mut self) {

        let rest = unsafe { tcsetattr(self.fd, TCSANOW, &self.original_termios) };
        if rest != 0 {
            panic!(
                "Failed to restore terminal {}: {}", 
                self.fd, 
                std::io::Error::last_os_error()
            );
        }
    }
}