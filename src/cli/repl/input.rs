pub mod termguard;

use std::os::unix::io::AsRawFd;

pub fn session() {
    
    let fd = std::io::stdin().as_raw_fd();
    let _guard = termguard::TermiosGuard::new(fd).unwrap();
    _guard.enable_raw_mode().unwrap();

}