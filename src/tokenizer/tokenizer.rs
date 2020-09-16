use super::token::Token;
use super::{
    is_bom_start, is_digit, is_hex_digit, is_identifier_start, is_name, is_name_start, is_newline,
    is_non_printable, is_valid_escape, is_whitespace,
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
        let code = self.byte(0);
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
            _ => {
                if is_name_start(code) {
                    self.consume_ident_like()
                } else {
                    let offset = self.pos;
                    self.forward(1);
                    Token::Delim(self.get_str(offset))
                }
            }
        }
    }
    #[inline]
    fn consume_simple(&mut self, t: Token<'a>) -> Token {
        self.pos += 1;
        t
    }
    // find whitespace end
    fn find_whitespace_end(&mut self) {
        while !self.is_eof() {
            let byte = self.byte(0);
            if is_whitespace(byte) {
                self.forward(1);
            } else {
                break;
            }
        }
    }
    // § 4.3.4. Consume an ident-like token
    // https://drafts.csswg.org/css-syntax/#consume-ident-like-token
    fn consume_ident_like(&mut self) -> Token {
        let offset = self.pos;
        self.consume_name();
        let s = self.get_str(offset);
        let byte = self.byte(0);
        // If string’s value is an ASCII case-insensitive match for "url",
        // and the next input code point is U+0028 LEFT PARENTHESIS ((), consume it.
        if s.to_ascii_lowercase() == "url" && byte == 0x0028 {
            self.forward(1);
            // While the next two input code points are whitespace, consume the next input code point.
            self.find_whitespace_end();
            let byte = self.byte(0);
            // If the next one or two input code points are U+0022 QUOTATION MARK ("), U+0027 APOSTROPHE ('),
            // or whitespace followed by U+0022 QUOTATION MARK (") or U+0027 APOSTROPHE ('),
            // then create a <function-token> with its value set to string and return it.
            if byte == 0x0022 || byte == 0x0027 {
                self.set(offset + 4);
                return Token::Function(s);
            }
            return self.consume_url();
        }
        // Otherwise, if the next input code point is U+0028 LEFT PARENTHESIS ((), consume it.
        // Create a <function-token> with its value set to string and return it.
        if byte == 0x0028 {
            self.forward(1);
            return Token::Function(s);
        }
        // Otherwise, create an <ident-token> with its value set to string and return it.
        return Token::Ident(self.get_str(offset));
    }
    // § 4.3.6. Consume a url token
    // Note: This algorithm assumes that the initial "url(" has already been consumed.
    // This algorithm also assumes that it’s being called to consume an "unquoted" value, like url(foo).
    // A quoted value, like url("foo"), is parsed as a <function-token>. Consume an ident-like token
    // automatically handles this distinction; this algorithm shouldn’t be called directly otherwise.
    fn consume_url(&mut self) -> Token {
        self.find_whitespace_end();
        let offset = self.pos;
        while !self.is_eof() {
            let byte = self.byte(0);
            // U+0029 RIGHT PARENTHESIS ())
            if byte == 0x0029 {
                return Token::Url(self.get_str(offset));
            } else if is_whitespace(byte) {
                self.find_whitespace_end();
                if self.is_eof() {
                    return Token::Url(self.get_str(offset));
                }
                let code = self.byte(0);
                if code == 0x0029 {
                    return Token::Url(self.get_str(offset));
                }
                self.consume_bad_url_remnants();
                return Token::BadUrl(self.get_str(offset));
            } else if byte == 0x0022 || byte == 0x0027 || byte == 0x0028 || is_non_printable(byte) {
                self.consume_bad_url_remnants();
                return Token::BadUrl(self.get_str(offset));
            } else if byte == 0x005C {  // U+005C REVERSE SOLIDUS (\)
                let next = self.byte(1);
                if is_valid_escape(byte, next) {
                    self.consume_escaped();
                    return Token::Url(self.get_str(offset));
                }
                self.consume_bad_url_remnants();
                return Token::BadUrl(self.get_str(offset));
            }
            self.forward(1);
        }
        return Token::Url("");
    }
    // § 4.3.14. Consume the remnants of a bad url
    // ... its sole use is to consume enough of the input stream to reach a recovery point
    // where normal tokenizing can resume.
    fn consume_bad_url_remnants(&mut self) {
        while !self.is_eof() {
            self.forward(1);
            let byte = self.byte(0);
            if byte == 0x0029 {
                return;
            }
            let next = self.byte(1);
            if is_valid_escape(byte, next) {
                // Consume an escaped code point.
                // Note: This allows an escaped right parenthesis ("\)") to be encountered
                // without ending the <bad-url-token>. This is otherwise identical to
                // the "anything else" clause.
                self.consume_escaped();
            }
        }
    }
    // § 4.3.3. Consume a numeric token
    fn consume_numberic(&mut self) -> Token {
        let offset = self.pos;
        self.consume_number();
        let byte = self.byte(0);
        let next1 = self.byte(1);
        let next2 = self.byte(2);
        // If the next 3 input code points would start an identifier, then:
        if is_identifier_start(byte, next1, next2) {
            // Create a <dimension-token> with the same value and type flag as number, and a unit set initially to the empty string.
            // Consume a name. Set the <dimension-token>’s unit to the returned value.
            // Return the <dimension-token>.
            self.consume_name();
            return Token::Dimension(self.get_str(offset));
        }
        // Otherwise, if the next input code point is U+0025 PERCENTAGE SIGN (%), consume it.
        if byte == 0x0025 {
            self.forward(1);
            return Token::Percentage(self.get_str(offset));
        }
        Token::Number(self.get_str(offset))
    }
    // §4.3.12. Consume a number
    // 1, 1.0, 1.3e1, 1.3e+1, 1.3e-1
    fn consume_number(&mut self) {
        let mut byte = self.byte(0);
        // 2. If the next input code point is U+002B PLUS SIGN (+) or U+002D HYPHEN-MINUS (-),
        // consume it and append it to repr.
        if byte == 0x002B || byte == 0x002D {
            self.forward(1);
            byte = self.byte(0);
        }
        if is_digit(byte) {
            self.find_decimal_number_end();
            byte = self.byte(0);
        }
        // 4. If the next 2 input code points are U+002E FULL STOP (.) followed by a digit, then:
        let next = self.byte(1);
        if byte == 0x002E && is_digit(next) {
            self.forward(2);
            byte = self.byte(0);
            self.find_decimal_number_end();
        }
        // e or E
        if byte == 0x0045 || byte == 0x0065 {
            self.forward(1);
            byte = self.byte(0);
            // ... optionally followed by U+002D HYPHEN-MINUS (-) or U+002B PLUS SIGN (+) ...
            if byte == 0x002D || byte == 0x002B {
                self.forward(1);
                byte = self.byte(0);
            }
            // 5.4 While the next input code point is a digit, consume it and append it to repr.
            if is_digit(byte) {
                self.find_decimal_number_end();
            }
        }
    }
    fn find_decimal_number_end(&mut self) {
        while !self.is_eof() {
            let byte = self.byte(0);
            if !is_digit(byte) {
                break;
            }
            self.forward(1);
        }
    }
    // \
    fn consume_reverse_solidus(&mut self) -> Token {
        let offset = self.pos;
        let byte = self.byte(0);
        let next = self.byte(1);
        if is_valid_escape(byte, next) {
            self.consume_ident_like();
            Token::Delim(self.get_str(offset))
        } else {
            self.forward(1);
            Token::Delim(self.get_str(offset))
        }
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
    fn consume_hash(&mut self) -> Token {
        let offset = self.pos;
        let next1 = self.byte(1);
        let next2 = self.byte(2);
        self.forward(1);
        if is_name(next1) || is_valid_escape(next1, next2) {
            self.consume_name();
            return Token::Hash(self.get_str(offset));
        }
        return Token::Delim(self.get_str(offset));
    }
    // §4.3.11. Consume a name
    // Note: This algorithm does not do the verification of the first few code points that are necessary
    // to ensure the returned code points would constitute an <ident-token>. If that is the intended use,
    // ensure that the stream starts with an identifier before calling this algorithm.
    fn consume_name(&mut self) {
        while !self.is_eof() {
            let byte = self.byte(0);
            if is_name(byte) {
                self.forward(1);
                continue;
            }
            let next = self.byte(1);
            if is_valid_escape(byte, next) {
                self.consume_escaped();
                continue;
            }
            break;
        }
    }
    // § 4.3.7. Consume an escaped code point
    fn consume_escaped(&mut self) {
        // It assumes that the U+005C REVERSE SOLIDUS (\) has already been consumed and
        // that the next input code point has already been verified to be part of a valid escape.
        self.forward(2);
        // hex digit, \0001
        if is_hex_digit(self.byte(-1)) {
            // Consume as many hex digits as possible, but no more than 5.
            // Note that this means 1-6 hex digits have been consumed in total.
            let mut i = 0;
            while i < 5 {
                i += 1;
                self.forward(1);
                let byte = self.byte(0);
                if !is_hex_digit(byte) {
                    break;
                }
            }
        }
    }
    #[inline]
    fn get_str(&self, offset: usize) -> &'a str {
        str::from_utf8(&self.input[offset..self.pos]).expect("")
    }
    fn byte(&self, idx: isize) -> u8 {
        let index = (self.pos as isize + idx) as usize;
        if index >= self.input.len() {
            0
        } else {
            self.input[index]
        }
    }
    #[inline]
    fn forward(&mut self, step: usize) {
        self.pos += step;
    }
    fn set(&mut self, pos: usize) {
        self.pos = pos;
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
