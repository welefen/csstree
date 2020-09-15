use super::source_location::{Position, SourceLocation};
use super::token::Token;
use super::{
    is_bom_start, is_digit, is_name, is_name_start, is_newline, is_valid_escape, is_whitespace,
};
use std::str;

pub struct Tokenizer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        let bytes = input.as_bytes();
        let mut pos = 0;
        if is_bom_start(bytes) {
            pos = 3;
        }
        Tokenizer { input: bytes, pos }
    }
    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }
    pub fn next(&mut self) -> Token {
        if self.is_eof() {
            return Token::EOF;
        }
        let code = self.input[self.pos];
        if is_whitespace(code) {
            return self.consume_whitespace();
        } else if is_digit(code) {
            return self.consume_numberic();
        } else if code >= 0x80 {
            return self.consume_ident_like();
        }
        match code {
            //  ' "
            0x0022 | 0x0027 => self.consume_string(),
            // #
            0x0023 => self.consume_hash(), 
            // &
            0x0026 => self.consume_simple(Token::And),
            // U+0028 LEFT PARENTHESIS (()
            0x0028 => self.consume_simple(Token::LeftParenthesis),
            // U+0029 RIGHT PARENTHESIS ())
            0x0029 => self.consume_simple(Token::RightParenthesis),
            // U+002B PLUS SIGN (+)
            0x002B => self.consume_plus(),
            // U+002C COMMA (,)
            0x002C => self.consume_simple(Token::Comma),
            // U+002D HYPHEN-MINUS (-)
            0x002D => self.consume_minus(),
            // U+002E FULL STOP (.)
            0x002E => self.consume_full_stop(),
            // U+002F SOLIDUS (/)
            0x002F => self.consume_solidus(),
            // U+003A COLON (:)
            0x003A => self.consume_simple(Token::Colon),
            // U+003B SEMICOLON (;)
            0x003B => self.consume_simple(Token::Semicolon),
            // U+003C LESS-THAN SIGN (<)
            0x003C => self.consume_less_than_sign(),
            // U+0040 COMMERCIAL AT (@)
            0x0040 => self.consume_at(),
            // U+005B LEFT SQUARE BRACKET ([)
            0x005B => self.consume_simple(Token::LeftSquareBracket),
            // U+005C REVERSE SOLIDUS (\)
            0x005C => self.consume_reverse_solidus(),
            // U+005D RIGHT SQUARE BRACKET (])
            0x005D => self.consume_simple(Token::RightSquareBracket),
            // U+007B LEFT CURLY BRACKET ({)
            0x007B => self.consume_simple(Token::LeftCurlyBracket),
            // U+007D RIGHT CURLY BRACKET (})
            0x007D => self.consume_simple(Token::RightCurlyBracket),
            _ => self.consume_simple(Token::Delim("")),
        }
    }
    fn consume_simple(&mut self, t: Token<'a>) -> Token {
        self.pos += 1;
        t
    }
    fn consume_ident_like(&self) -> Token {
        return Token::Number("");
    }
    fn consume_numberic(&self) -> Token {
        return Token::Number("");
    }
    fn consume_reverse_solidus(&self) -> Token {
        return Token::Number("");
    }
    fn consume_at(&self) -> Token {
        return Token::Number("");
    }
    fn consume_less_than_sign(&self) -> Token {
        return Token::Number("");
    }
    fn consume_solidus(&self) -> Token {
        return Token::Number("");
    }
    fn consume_full_stop(&self) -> Token {
        return Token::Number("");
    }
    fn consume_minus(&self) -> Token {
        return Token::Number("");
    }
    fn consume_plus(&self) -> Token {
        return Token::Number("");
    }
    fn consume_string(&self) -> Token {
        return Token::String("");
    }
    fn consume_hash(&self) -> Token {
        let offset = self.pos;
        let next1 = self.byte(1);
        let next2 = self.byte(2);
        // if is_name(next1) || is_valid_escape(next1, next2) {}
        return Token::Hash("");
    }
    // §4.3.11. Consume a name
    // Note: This algorithm does not do the verification of the first few code points that are necessary
    // to ensure the returned code points would constitute an <ident-token>. If that is the intended use,
    // ensure that the stream starts with an identifier before calling this algorithm.
    fn consume_name() {

    }
    fn get_str(&self, offset: usize) -> &'a str {
        str::from_utf8(&self.input[offset..self.pos]).expect("")
    }
    fn byte(&self, idx: usize) -> u8 {
        let index = self.pos + idx;
        if index >= self.input.len() {
            0
        } else {
            self.input[index]
        }
    }
    // Consume as much whitespace as possible. Return a <whitespace-token>.
    fn consume_whitespace(&mut self) -> Token {
        let offset = self.pos;
        while !self.is_eof() {
            self.pos += 1;
            if !is_whitespace(self.input[self.pos]) {
                break;
            }
        }
        Token::WhiteSpace(self.get_str(offset))
    }
}
