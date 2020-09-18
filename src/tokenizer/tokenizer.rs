use super::token::Token;
use super::{
    is_digit, is_hex_digit, is_identifier, is_identifier_start, is_newline, is_non_printable,
    is_valid_escape, is_whitespace, would_start_an_identifier, would_start_a_number
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
        if self.is_eof() {
            0
        } else {
            self.input.as_bytes()[self.position]
        }
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
        &self.input[pos..self.position]
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
        } else if is_digit(code) {
            return self.consume_numberic();
        } else if is_identifier_start(code) {
            return self.consume_ident_like();
        }
        match code {
            b'"' | b'\'' => self.consume_string(),
            b'#' => self.consume_hash(),
            b'(' => self.consume_simple(Token::LeftParenthesis),
            b')' => self.consume_simple(Token::RightParenthesis),
            b'+' => self.consume_plus_sign(),
            b',' => self.consume_simple(Token::Comma),
            b'-' => self.consume_hyphen_minus(),
            b'.' => self.consume_full_stop(),
            b':' => self.consume_simple(Token::Colon),
            b';' => self.consume_simple(Token::Semicolon),
            b'<' => self.consume_less_than(),
            b'@' => self.consume_at(),
            b'[' => self.consume_simple(Token::LeftSquareBracket),
            b'\\' => self.consume_reverse_solidus(),
            b']' => self.consume_simple(Token::RightSquareBracket),
            b'{' => self.consume_simple(Token::LeftCurlyBracket),
            b'}' => self.consume_simple(Token::RightCurlyBracket),
            _ => {
                self.forward(1);
                Token::Delim(self.slice_str())
            }
        }
    }
    fn consume_reverse_solidus(&mut self) -> Token {
        let byte = self.byte();
        if is_valid_escape(byte, self.next(1)) {
            return self.consume_ident_like();
        }
        self.forward(1);
        Token::Delim(self.slice_str())
    }
    fn consume_at(&mut self) -> Token {
        self.forward(1);
        if would_start_an_identifier(self.byte(), self.next(1), self.next(2)) {
            self.consume_identifier();
            return Token::AtKeyword(self.slice_str());
        }
        Token::Delim(self.slice_str())
    }
    fn consume_less_than(&mut self) -> Token {
        self.forward(1);
        // <!--
        if self.byte() == b'!' && self.next(1) == b'-' && self.next(2) == b'-' {
            self.forward(3);
            return Token::CDO;
        }
        Token::Delim(self.slice_str())
    }
    fn consume_full_stop(&mut self) -> Token {
        let byte = self.byte();
        if would_start_a_number(byte, self.next(1), self.next(2)) {
            return self.consume_numberic();
        }
        self.forward(1);
        Token::Delim(self.slice_str())
    }
    fn consume_hyphen_minus(&mut self) -> Token {
        let byte = self.byte();
        let next1 = self.next(1);
        let next2 = self.next(2);
        // -2
        if would_start_a_number(byte, next1, next2) {
            return self.consume_numberic();
        }
        // -->
        if next1 == b'-' && next2 == b'>' {
            self.forward(2);
            return Token::CDC;
        }
        if would_start_an_identifier(byte, next1, next2) {
            return self.consume_ident_like();
        }
        self.forward(1);
        return Token::Delim(self.slice_str());
    }
    // https://drafts.csswg.org/css-syntax/#consume-ident-like-token
    fn consume_ident_like(&mut self) -> Token {
        let pos = self.position;
        self.consume_identifier();
        let s = &self.input[pos..self.position];
        let url = "url";
        if s.eq_ignore_ascii_case("url") {
            if self.byte() == b'(' {
                self.forward(1);
            }
            self.forward_to_whitespace_end();
            let byte = self.byte();
            if byte == b'\'' || byte == b'"' {
                return Token::Function(url);
            }
            return self.consume_url();
        }
        if self.byte() == b'(' {
            self.forward(1);
            return Token::Function(s);
        }
        return Token::Ident(s);
    }
    // https://drafts.csswg.org/css-syntax/#consume-a-url-token
    fn consume_url(&mut self) -> Token {
        self.forward_to_whitespace_end();
        let pos = self.position;
        while !self.is_eof() {
            let byte = self.byte();
            if byte == b')' {
                break;
            } else if is_whitespace(byte) {
                self.forward_to_whitespace_end();
                if self.is_eof() || self.byte() == b')' {
                    break;
                } else {
                    self.consume_bad_url_remnants();
                    return Token::BadUrl(self.slice_str_pos(pos));
                }
            } else if byte == b'"' || byte == b'\'' || byte == b'(' || is_non_printable(byte) {
                self.consume_bad_url_remnants();
                return Token::BadUrl(self.slice_str_pos(pos));
            } else  if byte == b'\\' {
                if is_valid_escape(byte, self.next(1)) {
                    self.consume_escaped();
                } else {
                    self.consume_bad_url_remnants();
                    return Token::BadUrl(self.slice_str_pos(pos));
                }
            } else {
                self.forward(1);
            }
        }
        Token::Url(self.slice_str_pos(pos))
    }
    // https://drafts.csswg.org/css-syntax/#consume-remnants-of-bad-url
    fn consume_bad_url_remnants(&mut self) {
        while !self.is_eof() {
            let byte = self.byte();
            if byte == b')' {
                break;
            }
            if is_valid_escape(byte, self.next(1)) {
                self.consume_escaped();
            } else {
                self.forward(1);
            }
        }
    }
    fn consume_plus_sign(&mut self) -> Token {
        if would_start_a_number(self.byte(), self.next(1), self.next(2)) {
            return self.consume_numberic();
        }
        self.forward(1);
        Token::Delim(self.slice_str())
    }
    fn forward_to_decimal_end(&mut self) {
        while !self.is_eof() {
            let byte = self.byte();
            if is_digit(byte) {
                self.forward(1);
            } else {
                break;
            }
        }
    }
    fn forward_to_whitespace_end(&mut self) {
        while !self.is_eof() {
            let byte = self.byte();
            if is_whitespace(byte) {
                self.forward(1);
            } else {
                break;
            }
        }
    }
    // https://drafts.csswg.org/css-syntax/#consume-a-number
    fn consume_number(&mut self) {
        let mut byte = self.byte();
        if byte == b'+' || byte == b'-' {
            self.forward(1);
        }
        self.forward_to_decimal_end();
        if self.byte() == b'.' {
            self.forward(1);
            self.forward_to_decimal_end();
        }
        byte = self.byte();
        if byte == b'E' || byte == b'e' {
            self.forward(1);
            byte = self.byte();
            if byte == b'+' || byte == b'-' {
                self.forward(1);
            }
            self.forward_to_decimal_end();
        }
    }
    // https://drafts.csswg.org/css-syntax/#consume-a-numeric-token
    fn consume_numberic(&mut self) -> Token {
        self.consume_number();
        let byte = self.byte();
        if would_start_an_identifier(byte, self.next(1), self.next(2)) {
            self.consume_identifier();
            Token::Dimension(self.slice_str())
        } else if byte == b'%' {
            self.forward(1);
            Token::Percentage(self.slice_str())
        } else {
            Token::Number(self.slice_str())
        }
    }
    fn consume_simple(&mut self, t: Token<'a>) -> Token {
        self.forward(1);
        t
    }
    // https://drafts.csswg.org/css-syntax/#consume-token
    fn consume_hash(&mut self) -> Token {
        self.forward(1);
        let byte = self.byte();
        if is_identifier(byte) || is_valid_escape(byte, self.next(1)) {
            // let third = self.next(2);
            // if would_start_an_identifier(byte, next, third) {

            // }
            self.consume_identifier();
            Token::Hash(self.slice_str())
        } else {
            Token::Delim(self.slice_str())
        }
    }
    // https://drafts.csswg.org/css-syntax/#consume-an-identifier
    fn consume_identifier(&mut self) {
        while !self.is_eof() {
            let byte = self.byte();
            if is_identifier(byte) {
                self.forward(1);
                continue;
            }
            let next = self.next(1);
            if is_valid_escape(byte, next) {
                self.consume_escaped();
                continue;
            }
            break;
        }
    }
    // https://drafts.csswg.org/css-syntax/#consume-comments
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
    // https://drafts.csswg.org/css-syntax/#whitespace
    fn consume_whitespace(&mut self) -> Token {
        self.forward_to_whitespace_end();
        Token::WhiteSpace(self.slice_str())
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
    // @TODO  check numberic is valid
    // @TODO skip whitespace when is \r\n
    fn consume_escaped(&mut self) {
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
