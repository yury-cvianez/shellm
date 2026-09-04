use libc::{
    tcgetattr,
    tcsetattr, 
    termios, 
    TCSANOW
};

pub struct TermiosGuard {
    fd: i32,
    original_termios: termios,
}

impl TermiosGuard {

    pub fn new(fd: i32) -> std::io::Result<Self> {

        let mut original_termios = unsafe { std::mem::zeroed() };

        if unsafe { tcgetattr(fd, &mut original_termios) } != 0 {
            return Err(std::io::Error::last_os_error());
        }        

        Ok(TermiosGuard{fd, original_termios})
    }

    pub fn enable_raw_mode(&self) -> std::io::Result<()> {

        let mut raw_termios = self.original_termios;

        raw_termios.c_lflag &= !(
            libc::ECHO    | // echoes typed characters
            libc::ICANON  | // line by line input 
            libc::ISIG      // disables signals especial characters
        );

        raw_termios.c_cc[libc::VMIN] = 1; // minium number of bytes before read returns
        raw_termios.c_cc[libc::VTIME] = 0; // timeout of read in tenths of a second

        if unsafe { tcsetattr(self.fd, TCSANOW, &raw_termios) } != 0 {
            return Err(std::io::Error::last_os_error());
        }        

        Ok(())
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