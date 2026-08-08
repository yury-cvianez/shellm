# shellm

This project enables LLMs to assist in executing operations within Unix command-line terminals.
It integrates features such as autocomplete, correction suggestions, and the automation of repetitive tasks; it enhances safety by previewing the command's effect and allows users to determine the correct command simply by describing their objective.

It’s a custom-built shell, so it isn’t fully feature-complete; it’s more of a hobby project—something to work on when you have free time or just to keep your brain active.

Well, the simplest shell workflow is:
    usuario digita
        |
    entrada é interpretada
        |
    execução é estruturada
        |
    kernel e OS é acionado
        |
    comando é executado. 


Now, still trying to keep it simple, it basically becomes:
    Entrada Texto
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
      Processos