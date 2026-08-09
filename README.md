# shellm

This project enables LLMs to assist in executing operations within Unix command-line terminals.
It integrates features such as autocomplete, correction suggestions, and the automation of repetitive tasks; it enhances safety by previewing the command's effect and allows users to determine the correct command simply by describing their objective.

It’s a custom-built shell, so it isn’t fully feature-complete; it’s more of a hobby project—something to work on when you have free time or just to keep your brain active.


Well, the simplest shell workflow is:
    user types
        |
    input is interpreted
        |
    execution is structured
        |
    kernel e OS is activated
        |
    Command is executed. 


Still trying to keep it simple, it basically becomes:
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


Going into a bit more detail, but keeping it simple:
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


Now, what each thing is and why:

- Text Input: 
refers to the keyboard—what the user types.

- line editor: 
this would be a programming language library handling interaction during text editing.

- Lexer: 
In a way, it organizes the text typed by the user; it maps the start and end of words and special characters—it’s as if it were breaking down an order.

- Parser: 
Identifies and structures the command; it validates the command (the sequence provided by the lexer), 
defines the command, and handles elements such as command arguments, redirections, etc.

- Path resolver: 
Works by locating the command's executable; it simply resolves 
and identifies the command—previously identified by the parser—on the machine.

- Executor: 
Separates the instructions to be executed and makes requests to the kernel.

- Kernel: 
Contains the mechanisms and executes the instructions.

- OS: 
Obviously, the system as a whole within which the shell operates in relation to the hardware.