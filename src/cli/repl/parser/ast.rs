use std::fmt;

#[derive(Debug)]
pub struct Pipeline {
    pub commands: Vec<Command>,
}

#[derive(Debug)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
    pub redirections: Vec<Redirection>,
}

#[derive(Debug)]
pub struct Redirection {
    pub kind: RedirectKind,
    pub target: String,
}

#[derive(Debug)]
pub enum RedirectKind {
    Stdout, // >
    Append, // >>
    Stdin,  // <
}

impl fmt::Display for RedirectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RedirectKind::Stdout => write!(f, ">"),
            RedirectKind::Append => write!(f, ">>"),
            RedirectKind::Stdin  => write!(f, "<"),
        }
    }
}