mod lexer;
mod utils;
mod expressions;
mod statement;
mod ast;

use std::env;
use std::fs;
use std::io::{self, Write};

// TODO: After implementing the lexer, create unit tests for each operation to make that all cases are being covered
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let mut file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            // Fuck windows
            file_contents = file_contents.replace("\r\n", "\n");
            lexer::Scanner::new(file_contents).scan_tokens();
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
