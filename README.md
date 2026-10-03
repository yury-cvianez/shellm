# sshelm


This project enables LLMs to assist in executing operations within Unix command-line terminals.
It integrates autocomplete with SLM. It’s a custom-built shell, so it isn’t fully feature-complete; it’s more of a hobby project, something to work on when you have free time or just to keep your brain active.


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

Still trying to keep it simple, it basically becomes:
```text
    Text Input
        |
      Lexer
        |
      Parser
        |
     Path Resolver
        |
     Executor
        |
      kernel
        |
        OS
        |
      Process
```

Going into a bit more detail, but keeping it simple:
```text
    Text Input
        | line editor
      Lexer
        | tokens; character sequencing
      Parser
        | structured intention
     Path Resolver
        | resolution
     Executor
        | operations
      kernel
        | mechanism 
        OS
        | machine/hardware execution
      Process 
```

Now, what each thing is and why:

- Text Input: 
refers to the keyboard—what the user types.

- line editor: 
handling interaction during text editing.

- Lexer: 
In a way, it maps the text typed by the user to form tokens—for instance, by defining the start and end of a word for our terminal.

- Parser: 
Identifies and structures the command; it validates the command (the sequence provided by the lexer), 
defines the command, and handles elements such as command arguments, redirections, etc.

- Path resolver: 
Works by locating the command's executable; it simply resolves 
and identifies the command.

- Executor: 
Separates the instructions to be executed and makes requests to the kernel.

- Kernel: 
Contains the mechanisms and executes the instructions.

- OS: 
Obviously, the system as a whole within which the shell operates in relation to the hardware.