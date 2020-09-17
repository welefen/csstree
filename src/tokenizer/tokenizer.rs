use super::token::Token;
use super::{
    is_digit, is_hex_digit, is_identifier_start, is_newline,
    is_non_printable, is_whitespace, is_valid_escape
};
use std::str;

pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
    offset: usize,
    line: usize,
    column: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer {
            input,
            position: 0,
            offset: 0,
            line: 1,
            column: 1,
        }
    }
    #[inline]
    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }
    #[inline]
    fn byte(&self) -> u8 {
        self.input.as_bytes()[self.position]
    }
    #[inline]
    fn next(&self, idx: usize) -> u8 {
        let pos = self.position + idx;
        if pos >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[pos]
        }
    }
    #[inline]
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.position..].starts_with(s)
    }
    #[inline]
    fn forward(&mut self, step: usize) {
        self.position += step;
    }
    #[inline]
    fn back(&mut self, step: usize) {
        self.position -= step;
    }
    #[inline]
    fn slice_str(&self) -> &str {
        &self.input[self.offset..self.position]
    }
    #[inline]
    fn slice_str_pos(&self, pos: usize) -> &str {
        &self.input[self.offset..pos]
    }
    pub fn nextToken(&mut self) -> Token {
        if self.is_eof() {
            return Token::EOF;
        }
        self.offset = self.position;
        let pos = self.position;
        if self.starts_with("/*") {
            return self.consume_comment();
        }
        let code = self.byte();
        if is_whitespace(code) {
            return self.consume_whitespace();
        }
        match code {
            b'"' => self.consume_string(),
            b'#' => self.consume_hash(),
            _ => self.consume_string()
            // b'#' => self.consume_hash()
        }
    }
    fn consume_hash(&mut self) -> Token {
        Token::Hash("")
    }
    fn consume_comment(&mut self) -> Token {
        self.forward(2);
        while !self.is_eof() {
            if self.starts_with("*/") {
                self.forward(2);
                break;
            }
            self.forward(1);
        }
        Token::Comment(self.slice_str())
    }
    fn consume_whitespace(&mut self) -> Token {
        self.forward(1);
        while !self.is_eof() {
            let byte = self.byte();
            if !is_whitespace(byte) {
                break;
            }
            self.forward(1);
        }
        Token::WhiteSpace(self.slice_str())
    }
    fn consume_newline(&mut self) {

    }
    // https://drafts.csswg.org/css-syntax/#consume-a-string-token
    fn consume_string(&mut self) -> Token {
        let s = self.byte();
        self.forward(1);
        while !self.is_eof() {
            let byte = self.byte();
            if s == byte {
                self.forward(1);
                break;
            } else if is_newline(byte) {
                self.back(1);
                return Token::BadString(self.slice_str());
            } else if byte == b'\\' {
                let next = self.next(1);
                if next == 0 {
                    break;
                } else if is_newline(next) {
                    self.forward(1);
                } else if is_valid_escape(byte, next) {
                    self.consume_escaped();
                }
            } else {
                self.forward(1);
            }
        }
        Token::String(self.slice_str())
    }
    // https://drafts.csswg.org/css-syntax/#consume-an-escaped-code-point
    fn consume_escaped(&mut self)  {
        self.forward(1);
        let byte = self.byte();
        self.forward(1);
        if is_hex_digit(byte) {
            let mut i = 0;
            while i < 5 && !self.is_eof() {
                let byte = self.byte();
                if !is_hex_digit(byte) {
                    break;
                }
                i += 1;
                self.forward(1);
            }
            let byte = self.byte();
            if is_whitespace(byte) {
                self.forward(1);
            }
        }
    }
}
