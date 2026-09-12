# Tunisian Pseudocode Compiler

A compiler for the pseudocode language used in Tunisian high-school computer science courses.

The goal is pretty simple: take the pseudocode students already write in school and turn it into something that can actually be executed.

I'm building this mainly as a personal project to learn more about **compilers, lexing, parsing, ASTs, and code generation**, while also making something useful for the Tunisian programming curriculum.

## Current status

This project is **still in development**.

A lot of the language is not implemented yet, and things may change as I work on the compiler. Don't expect every valid school exercise to compile correctly just yet.

Currently being worked on:

* Lexing / tokenization
* Parsing
* AST construction
* Variables and types
* Conditions
* Loops
* Functions and procedures
* Local and global variables
* Expressions and operators
* Error handling

## Example

The compiler is designed to understand code written in a style like:

```text
algorithme exemple

début
    s <-- 0

    pour i de 1 à n faire
        s <-- s + i
    fin_pour

    écrire(s)
fin
```

The syntax follows the conventions commonly used in Tunisian high-school pseudocode rather than trying to introduce a completely new language.

## Why?

I've been interested in programming and how things work under the hood for a long time, and I wanted to learn how a compiler actually works by building one myself.

This project is also an excuse to work with things I don't normally get to work with, especially parsing and language design.

## Built with

The compiler is currently written in **Rust**.

Some of the libraries used include:

* [logos](https://crates.io/crates/logos) for lexing
* Rust's standard library for the rest of the compiler infrastructure

## Roadmap

There's still a lot to do.

* [x] Basic lexer
* [x] Basic parser structure
* [x] AST structure
* [ ] Complete expressions
* [ ] Complete control structures
* [ ] Functions and procedures
* [ ] Proper scope handling
* [ ] Better compiler errors
* [ ] Code generation
* [ ] Runtime / standard functions
* [ ] More complete compatibility with the Tunisian curriculum
* [ ] Tests for real-world school exercises

The roadmap will probably change as the project grows.

## Contributing

The project is still pretty early, so I'm not actively looking for large contributions yet.

If you find a bug, have an idea, or notice something that doesn't match the pseudocode syntax used in Tunisian schools, feel free to open an issue.

## License

This project is **source-available**.

It is free to use for personal, educational, and other non-commercial purposes. Commercial or enterprise use requires a separate commercial license.

See [`LICENSE`](LICENSE) for the full terms.
