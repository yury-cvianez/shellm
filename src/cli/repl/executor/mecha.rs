use crate::cli::repl::parser::ast::{Pipeline, RedirectKind};
use crate::cli::repl::executor::phresolver::{PathResolver, PathResolverError};
use std::fs::{File, OpenOptions};
use std::process::{Child, Command as ProcessCommand, Stdio};



#[derive(Debug)]
pub enum ExecutorError {
    PathResolverFailed(PathResolverError),
    SpawnFailed(String),
    RedirectFailed(String),
    WaitFailed(String),
    PipesNotImplemented,
}

pub struct Executor {
    path_resolver: PathResolver,
}

impl Executor {

    pub fn new() -> Self {
        Executor {
            path_resolver: PathResolver::new(),
        }
    }

    pub fn execute(&self, pipeline: Pipeline) -> Result<(), ExecutorError> {

        if pipeline.commands.len() != 1 {
            return Err(ExecutorError::PipesNotImplemented);
        }

        let cmd = &pipeline.commands[0];

        //let mut process = ProcessCommand::new(&cmd.program);

        let program_path = self.path_resolver.resolve(&cmd.program)?;
        let mut process = ProcessCommand::new(&program_path);

        process.args(&cmd.args);


        for redirect in &cmd.redirections {

            match redirect.kind {

                RedirectKind::Stdout => {

                    let file = File::create(&redirect.target)
                        .map_err(|e| {
                            ExecutorError::RedirectFailed(e.to_string())
                        })?;

                    process.stdout(Stdio::from(file));
                }


                RedirectKind::Append => {

                    let file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&redirect.target)
                        .map_err(|e| {
                            ExecutorError::RedirectFailed(e.to_string())
                        })?;

                    process.stdout(Stdio::from(file));
                }


                RedirectKind::Stdin => {

                    let file = File::open(&redirect.target)
                        .map_err(|e| {
                            ExecutorError::RedirectFailed(e.to_string())
                        })?;

                    process.stdin(Stdio::from(file));
                }
            }
        }


        let mut child = process
            .spawn()
            .map_err(|e| ExecutorError::SpawnFailed(e.to_string()))?;


        child
            .wait()
            .map_err(|e| ExecutorError::WaitFailed(e.to_string()))?;


        Ok(())
    }
}

impl From<PathResolverError> for ExecutorError {

    fn from(error: PathResolverError) -> Self {
        ExecutorError::PathResolverFailed(error)
    }
}