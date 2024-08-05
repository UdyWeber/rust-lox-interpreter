use std::env;
use std::fmt::{Display, Formatter, Pointer};
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

    // Comments
    COMMENT,
    COMMENT_BLOCK,

    // File terminators
    EOF,
}

enum Literal {
    String(String),
    Number(f32, usize),
    NULL,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Literal::String(s) => {
                    s.to_string()
                }
                Literal::Number(f, p) => {
                    format!("{:>.*}", p, f)
                }
                Literal::NULL => {
                    String::from("null")
                }
            }
        )
    }
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            self.literal.to_string()
        )
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Literal, line: usize) -> Self {
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

        self.tokens
            .push(Token::new(TokenType::EOF, "", Literal::NULL, self.line));
        self.print_tokens();

        return self.tokens;
    }

    fn print_tokens(&self) {
        self.tokens.iter().for_each(|t| println!("{}", t));

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
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, Literal::NULL),
            ')' => self.add_token(TokenType::RIGHT_PAREN, Literal::NULL),
            '{' => self.add_token(TokenType::LEFT_BRACE, Literal::NULL),
            '}' => self.add_token(TokenType::RIGHT_BRACE, Literal::NULL),
            ',' => self.add_token(TokenType::COMMA, Literal::NULL),
            '.' => self.add_token(TokenType::DOT, Literal::NULL),
            '-' => self.add_token(TokenType::MINUS, Literal::NULL),
            '+' => self.add_token(TokenType::PLUS, Literal::NULL),
            ';' => self.add_token(TokenType::SEMICOLON, Literal::NULL),
            '*' => self.add_token(TokenType::STAR, Literal::NULL),
            '!' => {
                let t_type = if self.match_next('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(t_type, Literal::NULL)
            }
            '=' => {
                let t_type = if self.match_next('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(t_type, Literal::NULL)
            }
            '>' => {
                let t_type = if self.match_next('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(t_type, Literal::NULL)
            }
            '<' => {
                let t_type = if self.match_next('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(t_type, Literal::NULL)
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    let comment = &self.source.get(self.start + 2..self.current).unwrap();
                    self.add_token(TokenType::COMMENT, Literal::String(comment.to_string()));
                    return;
                } else if self.match_next('*') {
                    while !(self.peek() == '*' && self.peek_next() == '/') && !self.is_at_end() {
                        if self.peek() == '\n' {
                            self.line += 1;
                        }
                        self.advance();
                    }

                    self.current += 2;

                    let comment = &self.source.get(self.start + 2..self.current-2).unwrap();
                    self.add_token(TokenType::COMMENT_BLOCK, Literal::String(comment.to_string()));
                    return;
                }
                self.add_token(TokenType::SLASH, Literal::NULL)
            }
            '"' => {
                self.handle_string();
            }
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {}
            _ => {
                if is_digit(c) {
                    self.handle_number()
                } else if is_alpha_numeric(c) {
                    self.handle_identifier()
                } else {
                    self.fmt_error(&format!("Unexpected character: {}", c));
                }
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.chars().count() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn get_lexeme(&self) -> &str {
        self.source.get(self.start..self.current).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let lexeme = self.get_lexeme();
        let token = Token::new(token_type, lexeme, literal, self.line);
        self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn match_next(&mut self, char_expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != char_expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn fmt_error(&mut self, msg: &str) {
        eprintln!("[line {}] Error: {}", self.line, msg);
        self.has_errors = true;
    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.fmt_error("Unterminated string.");
        }
        self.advance();

        let literal = self.source.get(self.start + 1..self.current - 1).unwrap();
        self.add_token(TokenType::STRING, Literal::String(literal.to_string()));
    }

    fn handle_identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let identifier = self.get_lexeme();
        let token_type = match identifier {
            "and" => TokenType::AND,
            "class" => TokenType::CLASS,
            "else" => TokenType::ELSE,
            "false" => TokenType::FALSE,
            "fun" => TokenType::FUN,
            "for" => TokenType::FOR,
            "if" => TokenType::IF,
            "nil" => TokenType::NIL,
            "or" => TokenType::OR,
            "print" => TokenType::PRINT,
            "return" => TokenType::RETURN,
            "super" => TokenType::SUPER,
            "this" => TokenType::THIS,
            "true" => TokenType::TRUE,
            "var" => TokenType::VAR,
            "while" => TokenType::WHILE,
            _ => TokenType::IDENTIFIER,
        };

        self.add_token(token_type, Literal::NULL)
    }

    fn handle_number(&mut self) {
        // TODO: Remove later, just needed for codecrafters printing scheiße
        let mut precision = 0;

        while is_digit(self.peek()) {
            self.advance();
        }

        // Checks if number has additional decimals
        if self.peek() == '.' && is_digit(self.peek_next()) {
            let mut only_zero_decimals = true;
            self.advance();

            while is_digit(self.peek()) {
                if self.peek() != '0' {
                    only_zero_decimals = false;
                }

                self.advance();
                precision += 1;
            }

            if only_zero_decimals {
                precision = 1;
            }
        }

        // Tries to parse number
        let source_num = self.get_lexeme();
        let parsed_num = source_num.parse::<f32>();
        if parsed_num.is_err() {
            return self.fmt_error(&format!("Parsing error on token: {}", source_num));
        }

        if precision == 0 {
            precision += 1;
        }
        self.add_token(
            TokenType::NUMBER,
            Literal::Number(parsed_num.unwrap(), precision),
        )
    }
}

fn is_alpha(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

fn is_digit(val: char) -> bool {
    val >= '0' && val <= '9'
}

fn is_alpha_numeric(c: char) -> bool {
    return is_digit(c) || is_alpha(c);
}

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
            Scanner::new(file_contents).scan_tokens();
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
