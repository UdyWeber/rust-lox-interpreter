use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::{self, Write};
use std::process::exit;

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

    // File terminators
    EOF,
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
    has_errors: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            has_errors: false,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token::new(TokenType::EOF, "", None, self.line));
        self.print_tokens();

        return self.tokens;
    }

    fn print_tokens(&self) {
        self.tokens
            .iter()
            .for_each(|t| println!("{}", t));

        if self.has_errors {
            exit(65);
        }
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.current += 1;
        return c;
    }

    fn scan_token(&mut self) {
        let c = &self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),
            '!' => {
                let t_type = if self.match_next('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(t_type, None)
            },
            '=' => {
                let t_type = if self.match_next('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(t_type, None)
            },
            '>' => {
                let t_type = if self.match_next('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(t_type, None)
            },
            '<' => {
                let t_type = if self.match_next('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(t_type, None)
            },
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    // Todo: Uncomment when finishing codecrafters stuff
                    // println!("Found comment: {}", &self.source.get(&self.start + 2..&self.current - 1).unwrap());
                    return;
                }
                self.add_token(TokenType::SLASH, None)
            },
            '\n' => {
                self.line += 1
            },
            ' ' | '\r' | '\t' => {},
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", &self.line, c);
                self.has_errors = true;
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current)
            .unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let lexeme = self.source.get(self.start..self.current).unwrap();
        let token = Token::new(token_type, lexeme, literal, self.line);

        self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn match_next(&mut self, char_expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.peek() != char_expected { return false; }

        self.current += 1;
        return true;
    }
}

// TODO: After implementing the lexer, create unit tests for each operation to make
// that all cases are being covered
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

            Scanner::new(file_contents).scan_tokens();
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
