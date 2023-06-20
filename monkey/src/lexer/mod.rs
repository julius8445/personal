use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident,
    Int,
    // Operators
    Lt,
    Gt,
    Eq,
    NotEq,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    FowardSlash,
    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    Eof,
    // Identifiers + literals
    Ident(String),
    Int(String),
    // Operators
    Lt,
    Gt,
    Eq,
    NotEq,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    FowardSlash,
    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal(s) => write!(f, "{}", s),
            Token::Eof => write!(f, ""),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Int(s) => write!(f, "{}", s),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::FowardSlash => write!(f, "/"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Lbrace => write!(f, "{{"),
            Token::Rbrace => write!(f, "}}"),
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    ch: u8,
    input: Vec<u8>,
    position: usize,
    read_position: usize,   
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            ch: 0,
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
        };
        l.read_char();
        l
    }
}

impl Lexer {
    fn _peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } 

        self.input[self.read_position]
    }
    
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }
}