use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::{self, Write};
use crate::TokenType::UNEXPECTED;

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,

    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
    UNEXPECTED,
}

impl From<char> for TokenType {
    fn from(value: char) -> Self {
        match value {
            '(' => Self::LEFT_PAREN,
            ')' => Self::RIGHT_PAREN,
            '{' => Self::LEFT_BRACE,
            '}' => Self::RIGHT_BRACE,
            ',' => Self::COMMA,
            '.' => Self::DOT,
            '-' => Self::MINUS,
            '+' => Self::PLUS,
            ';' => Self::SEMICOLON,
            '*' => Self::STAR,
            _ => UNEXPECTED,
        }
    }
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let literal = if self.literal.is_none() { String::from("null") } else { self.literal.clone().unwrap() };

        write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal)
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Option<String>, line: usize) -> Self {
        Token {
            token_type,
            lexeme: String::from(lexeme),
            literal,
            line,
        }
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token::new(TokenType::EOF, "", None, self.line));
        return self.tokens;
    }

    fn advance(&mut self) -> char {
        let c = self.source
            .chars()
            .nth(self.current)
            .unwrap();
        self.current += 1;
        return c;
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        let token_type = TokenType::from(c);

        match c {
            '(' => self.add_token(token_type, None),
            ')' => self.add_token(token_type, None),
            '{' => self.add_token(token_type, None),
            '}' => self.add_token(token_type, None),
            ',' => self.add_token(token_type, None),
            '.' => self.add_token(token_type, None),
            '-' => self.add_token(token_type, None),
            '+' => self.add_token(token_type, None),
            ';' => self.add_token(token_type, None),
            '*' => self.add_token(token_type, None),
            _ => {}
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let lexeme = self.source.get(self.start..self.current).unwrap();
        let token = Token::new(token_type, lexeme, literal, self.line);

        self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

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

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let tokenizer = Scanner::new(file_contents);
                for x in tokenizer.scan_tokens() {
                    println!("{}", x);
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
