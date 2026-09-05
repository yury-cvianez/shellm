pub mod termguard;
pub mod entryb;

use std::os::unix::io::AsRawFd;

pub fn session() -> std::io::Result<()> {
    
    let fd = std::io::stdin().as_raw_fd();
    let _guard = termguard::TermiosGuard::new(fd).unwrap();
    _guard.enable_raw_mode().unwrap();

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

        entryb::process_entry_byte(byte);
    };

    Ok(())

}