use super::token::Token;
use super::{
    is_digit, is_hex_digit, is_identifier, is_identifier_start, is_newline, is_non_printable,
    is_valid_escape, is_whitespace, would_start_a_number, would_start_an_identifier, utf8_is_cont_byte,
};

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
    fn advance_update(&mut self, step: usize) {
        let mut i = 1;
        while i <= step {
            i += 1;
            self.position += 1;
            let byte = self.next(0);
            if byte == b'\n' || (byte == b'\r' && self.next(1) != b'\n') {
                self.line += 1;
                self.column = 1;
            } else if byte <= 0x7F || !utf8_is_cont_byte(byte) {
                self.column += 1;
            }
        }
    }
    #[inline]
    fn advance(&mut self, step: usize) {
        self.position += step;
        self.column += step;
    }
    #[inline]
    fn slice_str(&self) -> &'a str {
        &self.input[self.offset..self.position]
    }
    #[inline]
    fn slice_str_pos(&self, pos: usize) -> &'a str {
        &self.input[pos..self.position]
    }
    /// get next token
    pub fn next_token(&mut self) -> Token<'a> {
        if self.is_eof() {
            return Token::EOF;
        }
        self.offset = self.position;
        let code = self.byte();
        if code == b'/' && self.next(1) == b'*' {
            return self.consume_comment();
        } else if is_whitespace(code) {
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
                self.advance(1);
                Token::Delim(self.slice_str())
            }
        }
    }
    fn consume_reverse_solidus(&mut self) -> Token<'a> {
        let byte = self.byte();
        if is_valid_escape(byte, self.next(1)) {
            return self.consume_ident_like();
        }
        self.advance(1);
        Token::Delim(self.slice_str())
    }
    fn consume_at(&mut self) -> Token<'a> {
        self.advance(1);
        if would_start_an_identifier(self.byte(), self.next(1), self.next(2)) {
            self.consume_identifier();
            return Token::AtKeyword(self.slice_str());
        }
        Token::Delim(self.slice_str())
    }
    fn consume_less_than(&mut self) -> Token<'a> {
        self.advance(1);
        // <!--
        if self.next(0) == b'!' && self.next(1) == b'-' && self.next(2) == b'-' {
            self.advance(3);
            return Token::CDO;
        }
        Token::Delim(self.slice_str())
    }
    fn consume_full_stop(&mut self) -> Token<'a> {
        let byte = self.byte();
        if would_start_a_number(byte, self.next(1), self.next(2)) {
            return self.consume_numberic();
        }
        self.advance(1);
        Token::Delim(self.slice_str())
    }
    fn consume_hyphen_minus(&mut self) -> Token<'a> {
        let byte = self.byte();
        let next1 = self.next(1);
        let next2 = self.next(2);
        // -2
        if would_start_a_number(byte, next1, next2) {
            return self.consume_numberic();
        }
        // -->
        if next1 == b'-' && next2 == b'>' {
            self.advance(2);
            return Token::CDC;
        }
        if would_start_an_identifier(byte, next1, next2) {
            return self.consume_ident_like();
        }
        self.advance(1);
        return Token::Delim(self.slice_str());
    }
    // https://drafts.csswg.org/css-syntax/#consume-ident-like-token
    fn consume_ident_like(&mut self) -> Token<'a> {
        let pos = self.position;
        self.consume_identifier();
        let s = &self.input[pos..self.position];
        let url = "url";
        if s.eq_ignore_ascii_case(url) {
            if self.next(0) == b'(' {
                self.advance(1);
            }
            self.advance_to_whitespace_end();
            let byte = self.next(0);
            if byte == b'\'' || byte == b'"' {
                return Token::Function(url);
            }
            return self.consume_url();
        }
        if self.next(0) == b'(' {
            self.advance(1);
            return Token::Function(s);
        }
        return Token::Ident(s);
    }
    // https://drafts.csswg.org/css-syntax/#consume-a-url-token
    fn consume_url(&mut self) -> Token<'a> {
        self.advance_to_whitespace_end();
        let pos = self.position;
        while !self.is_eof() {
            let byte = self.byte();
            if byte == b')' {
                break;
            } else if is_whitespace(byte) {
                self.advance_to_whitespace_end();
                if self.is_eof() || self.byte() == b')' {
                    break;
                } else {
                    self.consume_bad_url_remnants();
                    return Token::BadUrl(self.slice_str_pos(pos));
                }
            } else if byte == b'"' || byte == b'\'' || byte == b'(' || is_non_printable(byte) {
                self.consume_bad_url_remnants();
                return Token::BadUrl(self.slice_str_pos(pos));
            } else if byte == b'\\' {
                if is_valid_escape(byte, self.next(1)) {
                    self.consume_escaped();
                } else {
                    self.consume_bad_url_remnants();
                    return Token::BadUrl(self.slice_str_pos(pos));
                }
            } else {
                self.advance_update(1);
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
                self.advance_update(1);
            }
        }
    }
    fn consume_plus_sign(&mut self) -> Token<'a> {
        if would_start_a_number(self.byte(), self.next(1), self.next(2)) {
            return self.consume_numberic();
        }
        self.advance(1);
        Token::Delim(self.slice_str())
    }
    fn advance_to_decimal_end(&mut self) {
        while !self.is_eof() {
            let byte = self.byte();
            if is_digit(byte) {
                self.advance(1);
            } else {
                break;
            }
        }
    }
    fn advance_to_whitespace_end(&mut self) {
        while !self.is_eof() {
            let byte = self.byte();
            if is_whitespace(byte) {
                self.advance_update(1);
            } else {
                break;
            }
        }
    }
    // https://drafts.csswg.org/css-syntax/#consume-a-number
    fn consume_number(&mut self) {
        let mut byte = self.byte();
        if byte == b'+' || byte == b'-' {
            self.advance(1);
        }
        self.advance_to_decimal_end();
        if self.next(0) == b'.' {
            self.advance(1);
            self.advance_to_decimal_end();
        }
        byte = self.next(0);
        if byte == b'E' || byte == b'e' {
            self.advance(1);
            byte = self.next(0);
            if byte == b'+' || byte == b'-' {
                self.advance(1);
            }
            self.advance_to_decimal_end();
        }
    }
    // https://drafts.csswg.org/css-syntax/#consume-a-numeric-token
    fn consume_numberic(&mut self) -> Token<'a> {
        self.consume_number();
        let byte = self.next(0);
        if would_start_an_identifier(byte, self.next(1), self.next(2)) {
            self.consume_identifier();
            Token::Dimension(self.slice_str())
        } else if byte == b'%' {
            self.advance(1);
            Token::Percentage(self.slice_str())
        } else {
            Token::Number(self.slice_str())
        }
    }
    fn consume_simple(&mut self, t: Token<'a>) -> Token<'a> {
        self.advance(1);
        t
    }
    // https://drafts.csswg.org/css-syntax/#consume-token
    fn consume_hash(&mut self) -> Token<'a> {
        self.advance(1);
        let byte = self.next(0);
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
                self.advance(1);
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
    fn consume_comment(&mut self) -> Token<'a> {
        self.advance(2);
        while !self.is_eof() {
            if self.byte() == b'*' && self.next(1) == b'/' {
                self.advance(2);
                break;
            }
            self.advance_update(1);
        }
        Token::Comment(self.slice_str())
    }
    // https://drafts.csswg.org/css-syntax/#whitespace
    fn consume_whitespace(&mut self) -> Token<'a> {
        self.advance_to_whitespace_end();
        Token::WhiteSpace(self.slice_str())
    }
    // https://drafts.csswg.org/css-syntax/#consume-a-string-token
    fn consume_string(&mut self) -> Token<'a> {
        let s = self.byte();
        self.advance(1);
        while !self.is_eof() {
            let byte = self.byte();
            if s == byte {
                self.advance(1);
                break;
            } else if is_newline(byte) {
                return Token::BadString(self.slice_str());
            } else if byte == b'\\' {
                let next = self.next(1);
                if next == 0 {
                    break;
                } else if is_newline(next) {
                    self.advance_update(1);
                } else if is_valid_escape(byte, next) {
                    self.consume_escaped();
                }
            } else {
                self.advance_update(1);
            }
        }
        Token::String(self.slice_str())
    }
    // https://drafts.csswg.org/css-syntax/#consume-an-escaped-code-point
    // @TODO  check numberic is valid
    // @TODO skip whitespace when is \r\n
    fn consume_escaped(&mut self) {
        self.advance(1);
        let byte = self.next(0);
        self.advance(1);
        if is_hex_digit(byte) {
            let mut i = 0;
            while i < 5 && !self.is_eof() {
                let byte = self.byte();
                if !is_hex_digit(byte) {
                    break;
                }
                i += 1;
                self.advance(1);
            }
            let byte = self.next(0);
            if is_whitespace(byte) {
                self.advance_update(1);
            }
        }
    }
}
