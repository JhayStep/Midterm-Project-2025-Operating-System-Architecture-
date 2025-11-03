mod token;
mod ast;
mod lexer;
mod parser;
mod eval;
mod integrity;

use std::env;
use std::fs;

fn main() {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: calculator <path/to/file.lisp>");
            std::process::exit(1);
        }
    };

    let src = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => { eprintln!("Failed to read {}: {}", path, e); std::process::exit(1); }
    };

    let toks = match lexer::lex(&src) {
        Ok(t) => t,
        Err(e) => { eprintln!("Lex error: {}", e); std::process::exit(1); }
    };

    let ast = match parser::parse_tokens(toks) {
        Ok(a) => a,
        Err(e) => { eprintln!("Parse error: {}", e); std::process::exit(1); }
    };

    let val = match eval::eval(&ast) {
        Ok(v) => v,
        Err(e) => { eprintln!("Eval error: {}", e); std::process::exit(1); }
    };

    // keep numeric result as the last line for test scripts
    println!("{}", val);
}
