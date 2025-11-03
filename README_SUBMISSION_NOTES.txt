Course: CSC 385 – Operating Systems
Assignment: Midterm Project
Student: Jhaydn Steplight
Date: [Insert submission date]

Project Summary
This project implements a Lisp-style arithmetic calculator in Rust. The program reads Lisp expressions such as:
(+ (* 2 3) 4)
It then:
1) Lexes the input into tokens,
2) Parses those tokens into an Abstract Syntax Tree (AST),
3) Recursively evaluates the AST to produce a numeric result,
4) (Optional) Generates a hash for integrity verification.

What I Implemented
All required functions in the following modules were completed and tested:
- src/lexer.rs: tokenization and token classification
- src/parser.rs: recursive parsing into AST nodes
- src/eval.rs: recursive evaluation logic for +, -, *, /
- src/integrity.rs: SHA-256 hashing helper (optional use)
- src/token.rs and src/ast.rs: core data structures for tokens and AST nodes
- src/main.rs: CLI and pipeline (lex → parse → eval)

Testing
I ran bash test.sh and confirmed:
- The project builds with cargo build.
- Evaluation results match expectations for the provided test .lisp programs.
- The .tokens files match the lexer output when required by the tests.
- The program prints the numeric result on the last line so automated checks pass.

Submission Notes
- This directory layout matches the original midterm_project archive from Moodle.
- All required functions are implemented.
- This tarball (Jhaydn_midterm-project.tar.gz) is my final submission.
