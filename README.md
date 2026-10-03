# sshelm


This project enables SLMs to assist in executing operations within Unix command-line terminals.
It integrates autocomplete with SLM. It’s a custom-built shell, so it isn’t fully feature-complete; it’s more of a hobby project, something to work on when you have free time or just to keep your brain active.

---

### About
sshelm is intentionally built from the terminal input layer upward, implementing its:

* byte decoding;
* input processing;
* utf-8 support;
* sequence interpretation;
* control commands;
* a parser (tokenizer);
* a parser;
* path resolution;
* executor pipeline;

This was done because, during initialization, we save the current *termios*/tty terminal configuration and then modify it to enable *raw* mode—disabling canonical input processing and terminal echo—allowing *sshelm* to receive input byte-by-byte, interpret it, and transform it into an executable command. When the session ends, the
sshelm restores the original kernel's terminal configuration.

For autocomplete, sshelm uses **llama.cpp** as a local inference runtime/server. This decouples the model from the shell, allowing different compatible models to be used as the autocomplete engine without changing the shell's core, pressing TAB triggers the model and passes the current text state so it can provide suggestions.

---

Well, the simplest shell workflow is:
```text
    user types
        |
    input is interpreted
        |
    execution is structured
        |
    kernel e OS is activated
        |
    Command is executed. 
```

---

Still trying to keep it simple, it basically becomes:
```text
  Text Input
      |
  Line Editor
      |
  Lexer
      |
  Parser
      |
  Path Resolver
      |
  Executor
      |
  System Calls
      |
  Kernel
      |
  Process
```

---

Going into a bit more detail, but keeping it simple:
```text
    Text Input
        | line editor
      Lexer
        | tokens; character sequencing
      Parser
        | structured intention/command
     Path Resolver
        | resolution executable path
     Executor
        | operations/system calls
      kernel
        | mechanism / process / resources
      Process 
```

---

Now, what each thing is and why:

- Text Input: 
Refers to the keyboard—what the user types.

- line editor: 
Handles interactive command-line editing, cursor movement, control sequences, and UTF-8 input.

- Lexer: 
In a way, it maps the text typed by the user to form tokens—for instance, by defining the start and end of a word for our terminal. Converts the input text into tokens according to the shell's lexical rules

- Parser: 
Converts the token stream into a structured representation of the command line, validating its syntax and handling constructs such as arguments, pipes, and redirections.

- Path resolver: 
Resolves a command name to an executable path, including searching directories specified by PATH when necessary.

- Executor: 
Takes the structure produced by the parser, prepares the required process execution state, and invokes the operating system through system calls.

- Kernel: 
Provides the operating-system mechanisms used by the shell, such as process creation, program execution, file descriptors, pipes, and synchronization.s