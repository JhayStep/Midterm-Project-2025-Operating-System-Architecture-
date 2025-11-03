# Midterm Project — Operating System Architecture (Lisp calculator)

[![Rust CI](https://github.com/JhayStep/Midterm-Project-2025-Operating-System-Architecture-/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/JhayStep/Midterm-Project-2025-Operating-System-Architecture-/actions/workflows/rust-ci.yml)

This repository contains a small Lisp-style arithmetic calculator implemented in Rust for the CSC 385 midterm. The project lexes, parses and evaluates Lisp expressions and includes tests and a simple test script.

See `README_SUBMISSION_NOTES.txt` for submission details and implementation notes.

How to build (locally)

```sh
# build
cargo build --release

# run tests
cargo test

# run the provided test script (bash)
bash test.sh
```

If you're on Windows, use WSL or install the Visual C++ Build Tools for the MSVC toolchain.
