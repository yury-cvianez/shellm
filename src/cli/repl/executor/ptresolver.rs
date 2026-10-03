use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path};


pub struct PathResolver;


#[derive(Debug)]
pub enum PathResolverError {
    NotFound(String),
}


impl PathResolver {

    pub fn new() -> Self {
        PathResolver
    }


    pub fn resolve(&self, program: &str) -> Result<String, PathResolverError> {

        if program.contains('/') {
            let path = Path::new(program);

            if Self::_is_executable(path) {
                return Ok(program.to_string());
            }

            return Err(PathResolverError::NotFound(program.to_string()));
        }


        if let Ok(path_env) = env::var("PATH") {

            for dir in path_env.split(':') {

                let full_path = Path::new(dir).join(program);

                if Self::_is_executable(&full_path) {
                    return Ok(full_path.to_string_lossy().into_owned());
                }
            }
        }


        Err(PathResolverError::NotFound(program.to_string()))
    }


    fn _is_executable(path: &Path) -> bool {

        let metadata = match path.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return false,
        };


        metadata.is_file()
            && metadata.permissions().mode() & 0o111 != 0
    }
}