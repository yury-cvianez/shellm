use crate::cli::repl::frame::mold::Frame;
use crate::cli::repl::lexer::tokenizer::Token;
use crate::cli::repl::parser::ast::{Command, Pipeline};

pub fn completion(f: &mut Frame, typed: &str, completed: &str) {
    f.line(format!("[~] completion  \"{typed}\" -> \"{completed}\""));
}

pub fn tokens(f: &mut Frame, tokens: &[Token]) {
    let list: Vec<String> = tokens
        .iter()
        .map(|t| match t {
            Token::Word(w) => format!("\"{w}\""),
            op => op.to_string(),
        })
        .collect();
    f.line(format!("[1] tokens    {}", list.join("  ")));
}

pub fn parsed(f: &mut Frame, pipeline: &Pipeline) {
    let n = pipeline.commands.len();
    f.line(format!("[2] parse     {n} command{}", if n == 1 { "" } else { "s" }));

    for (i, cmd) in pipeline.commands.iter().enumerate() {
        let tag = if n > 1 { format!("#{} ", i + 1) } else { String::new() };
        command(f, &tag, cmd);
    }
}

fn command(f: &mut Frame, tag: &str, cmd: &Command) {
    let pad = " ".repeat(14);
    f.line(format!("{pad}{tag}program:  {}", cmd.program));

    let indent = format!("{pad}{}", " ".repeat(tag.len()));
    if !cmd.args.is_empty() {
        f.line(format!("{indent}args:     {}", cmd.args.join(" ")));
    }
    for r in &cmd.redirections {
        f.line(format!("{indent}redirect: {} {}", r.kind, r.target));
    }
}

pub fn execute(f: &mut Frame) {
    f.line("[3] execute");
    f.line("");
}

pub fn error(f: &mut Frame, kind: &str, err: &dyn std::fmt::Debug) {
    f.line(format!("[!] {kind}: {err:?}"));
}

pub fn eof() {
    println!("[end] EOF, shell closed");
}