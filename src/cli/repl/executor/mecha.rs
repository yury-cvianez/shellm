use crate::cli::repl::parser::ast::{Pipeline, RedirectKind};
use crate::cli::repl::executor::ptresolver::{
    PathResolver,
    PathResolverError,
};

use std::io::Read;
use std::fs::{File, OpenOptions};
use std::process::{Child, Command as ProcessCommand, Stdio};


pub struct Executor {
    path_resolver: PathResolver,
}


#[derive(Debug)]
pub enum ExecutorError {
    PathResolverFailed(PathResolverError),
    SpawnFailed(String),
    RedirectFailed(String),
    WaitFailed(String),
}


impl Executor {

    pub fn new() -> Self {
        Executor {
            path_resolver: PathResolver::new(),
        }
    }

    pub fn execute(&self, pipeline: Pipeline) -> Result<String, ExecutorError>  {

        let mut children: Vec<Child> =
            Vec::with_capacity(pipeline.commands.len());

        let mut previous_stdout = None;

        let (mut reader, writer) = std::io::pipe()
            .map_err(|e| ExecutorError::SpawnFailed(e.to_string()))?;

        for (index, cmd) in pipeline.commands.iter().enumerate() {

            let is_last =
                index == pipeline.commands.len() - 1;


            let program_path =
                self.path_resolver.resolve(&cmd.program)?;


            let mut process =
                ProcessCommand::new(&program_path);

            process.args(&cmd.args);


            if let Some(stdout) = previous_stdout.take() {
                process.stdin(Stdio::from(stdout));
            }


            process.stderr(Self::capture(&writer)?);
            if is_last {
                process.stdout(Self::capture(&writer)?);
            } else {
                process.stdout(Stdio::piped());
            }

            for redirect in &cmd.redirections {

                match redirect.kind {

                    RedirectKind::Stdout => {

                        let file =
                            File::create(&redirect.target)
                                .map_err(|e| {
                                    ExecutorError::RedirectFailed(
                                        e.to_string()
                                    )
                                })?;

                        process.stdout(Stdio::from(file));
                    }


                    RedirectKind::Append => {

                        let file =
                            OpenOptions::new()
                                .append(true)
                                .create(true)
                                .open(&redirect.target)
                                .map_err(|e| {
                                    ExecutorError::RedirectFailed(
                                        e.to_string()
                                    )
                                })?;

                        process.stdout(Stdio::from(file));
                    }


                    RedirectKind::Stdin => {

                        let file =
                            File::open(&redirect.target)
                                .map_err(|e| {
                                    ExecutorError::RedirectFailed(
                                        e.to_string()
                                    )
                                })?;

                        process.stdin(Stdio::from(file));
                    }
                }
            }

            let mut child =
                process
                    .spawn()
                    .map_err(|e| {
                        ExecutorError::SpawnFailed(
                            e.to_string()
                        )
                    })?;

            if !is_last {
                previous_stdout =
                    child.stdout.take();
            }


            children.push(child);
        }
        
        // CRÍTICO: fecha a nossa ponta de escrita, senão read_to_end nunca vê EOF
        drop(writer);

        // lê ANTES de esperar: se o filho encher o buffer do pipe, ele bloqueia
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .map_err(|e| ExecutorError::WaitFailed(e.to_string()))?;

        for mut child in children {
            child
                .wait()
                .map_err(|e| ExecutorError::WaitFailed(e.to_string()))?;
        }

        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }

    fn capture(writer: &std::io::PipeWriter) -> Result<Stdio, ExecutorError> {
        writer
            .try_clone()
            .map(Stdio::from)
            .map_err(|e| ExecutorError::SpawnFailed(e.to_string()))
    }

}


impl From<PathResolverError> for ExecutorError {

    fn from(error: PathResolverError) -> Self {
        ExecutorError::PathResolverFailed(error)
    }
}