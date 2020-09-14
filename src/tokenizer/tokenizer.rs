use super::source_location::{Position, SourceLocation};
use super::token::Token;
use super::{has_bom, is_digit, is_newline, is_whitespace};
use std::str;

pub struct Tokenizer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        let bytes = input.as_bytes();
        let mut pos = 0;
        if has_bom(bytes) {
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
        let byte = self.input[self.pos];
        if is_whitespace(byte) {
            return self.consume_whitespace();
        } else if is_digit(byte) {
            return self.consume_numberic();
        }
        match byte {
            //  ' "
            0x0022 | 0x0027 => self.consume_string(),
            0x0023 => self.consume_hash(), // #
            // U+0028 LEFT PARENTHESIS (()
            0x0028 => {
                self.pos += 1;
                Token::LeftParenthesis
            }
            // U+0029 RIGHT PARENTHESIS ())
            0x0029 => {
                self.pos += 1;
                Token::RightParenthesis
            }
            // U+002B PLUS SIGN (+)
            0x002B => self.consume_plus(),
            // U+002C COMMA (,)
            0x002C => {
                self.pos += 1;
                Token::Comma
            }
            // U+002D HYPHEN-MINUS (-)
            0x002D => self.consume_minus(),
            // U+002E FULL STOP (.)
            0x002E => self.consume_full_stop(),
            // U+002F SOLIDUS (/)
            0x002F => self.consume_solidus(),
            // U+003A COLON (:)
            0x003A => {
                self.pos += 1;
                Token::Colon
            }
            // U+003B SEMICOLON (;)
            0x003B => {
                self.pos += 1;
                Token::Semicolon
            }
            // U+003C LESS-THAN SIGN (<)
            0x003C => self.consume_less_than_sign(),
            // U+0040 COMMERCIAL AT (@)
            0x0040 => self.consume_at(),
            // U+005B LEFT SQUARE BRACKET ([)
            0x005B => {
                self.pos += 1;
                Token::LeftSquareBracket
            }
            // U+005C REVERSE SOLIDUS (\)
            0x005C => self.consume_reverse_solidus(),
            // U+005D RIGHT SQUARE BRACKET (])
            0x005D => {
                self.pos += 1;
                Token::RightSquareBracket
            }
            // U+007B LEFT CURLY BRACKET ({)
            0x007B => {
                self.pos += 1;
                Token::LeftCurlyBracket
            }
            // U+007D RIGHT CURLY BRACKET (})
            0x007D => {
                self.pos += 1;
                Token::RightSquareBracket
            }
            _ => {
                self.pos += 1;
                Token::Delim("")
            }
        }
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
        return Token::Hash("");
    }
    fn get_str(&self, offset: usize) -> &'a str {
        str::from_utf8(&self.input[offset..self.pos]).expect("")
    }
    fn consume_whitespace(&mut self) -> Token {
        let offset = self.pos;
        loop {
            self.pos += 1;
            if !is_whitespace(self.input[self.pos]) {
                break;
            }
        }
        Token::WhiteSpace(self.get_str(offset))
    }
}
