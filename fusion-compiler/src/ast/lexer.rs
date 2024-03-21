// use std::{iter::Peekable, str::Chars};

pub struct TokenKind {
    Number:i64,
    Plus: String,
    Minus: String,
    Asterisk: String,
    Slash: String,
    LeftParen: String,
    RightParen: String,
    Bad: String,
    Eof: String,
}

pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end : usize, literal : String) -> Self {
        Self{start, end, literal}
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

impl Token {
    pub fn new(kind : TokenKind, span : TextSpan) -> Self {
        Self {kind, span}
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl <'a> Lexer<'a> {

    pub fn new(input: &'a str) -> Self {
        Self {input, current_pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos > self.input.len() {
            return None;
        }

        if self.current_pos == self.input.len() {
            let eof_char: char = '\0';
            self.current_pos += 1;  
            return Some{Token::new{
                kind: TokenKind::Eof, 
                span: TextSpan::new{start: 0, end: 0, literal: eof_char.to_string()}
            }};
        }

        let start: usize = self.current_pos;
        let c: char = self.current_char();
        let mut kind = TokenKind::Bad;
        if Self::is_number_start(&c) {
            let number: i64 = self.consumer_number();
            kind = TokenKind::Number(number);
        }

        let end: usize = self.current_pos;
        let literal: String = self.input[start..end].to_string();
        let span: TextSpan = TextSpan::new(start, end, literal);
        Some(Token::new(kind, span))
    }

    fn is_number_start( c: &char) -> bool {
        c.is_digit( radix: 10)
    }

    fn current_char(&self) -> char {
        self.input.chars().nth( n: self.current_pos).unwrap()
    }
    fn consume(&mut self) -> Option<char> {
        let c: char = self.current_char();
        self.current_pos += 1;
        if self.current_pos > self.input.len() {
            return None;
        }
        Some(c)
    }

    fn consumer_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c ) = self.consume() {
            if c.is_digit( radix : 10) {
                number = number*10 + c.to_digit(radix: 10).unwrap() as i64;
            } else {
                break;
            }
        }
    }
}