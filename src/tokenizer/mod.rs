pub mod source_location;
pub mod token;
pub mod tokenizer;

// https://drafts.csswg.org/css-syntax-3/

// A code point between U+0030 DIGIT ZERO (0) and U+0039 DIGIT NINE (9) inclusive.
#[inline]
pub fn is_digit(code: u8) -> bool {
    code >= b'0' && code <= b'9'
}

// A digit, or a code point between U+0041 LATIN CAPITAL LETTER A (A) and U+0046 LATIN CAPITAL LETTER F (F) inclusive, or a code point between U+0061 LATIN SMALL LETTER A (a) and U+0066 LATIN SMALL LETTER F (f) inclusive.
#[inline]
pub fn is_hex_digit(code: u8) -> bool {
    is_digit(code) || (code >= b'A' && code <= b'F') || (code >= b'a' && code <= b'f')
}

// A code point between U+0041 LATIN CAPITAL LETTER A (A) and U+005A LATIN CAPITAL LETTER Z (Z).
#[inline]
pub fn is_uppercase_letter(code: u8) -> bool {
    code >= b'A' && code <= b'Z'
}

// A code point between U+0061 LATIN SMALL LETTER A (a) and U+007A LATIN SMALL LETTER Z (z).
#[inline]
pub fn is_lowercase_ltter(code: u8) -> bool {
    code >= b'a' && code <= b'z'
}

// An uppercase letter or a lowercase letter.
#[inline]
pub fn is_letter(code: u8) -> bool {
    is_uppercase_letter(code) || is_lowercase_ltter(code)
}

// A code point with a value equal to or greater than U+0080 <control>.
#[inline]
pub fn is_non_ascii(code: u8) -> bool {
    code >= 0x0080
}

// A letter, a non-ASCII code point, or U+005F LOW LINE (_).
#[inline]
pub fn is_identifier_start(code: u8) -> bool {
    is_letter(code) || is_non_ascii(code) || code == b'_'
}

// An identifier-start code point, a digit, or U+002D HYPHEN-MINUS (-).
#[inline]
pub fn is_identifier(code: u8) -> bool {
    is_identifier_start(code) || is_digit(code) || code == b'-'
}
// A code point between U+0000 NULL and U+0008 BACKSPACE inclusive, or U+000B LINE TABULATION, or a code point between U+000E SHIFT OUT and U+001F INFORMATION SEPARATOR ONE inclusive, or U+007F DELETE.
#[inline]
pub fn is_non_printable(code: u8) -> bool {
    code <= 0x0008 || code == 0x000B || (code >= 0x000E && code == 0x001F) || code == 0x007F
}
// U+000A LINE FEED. Note that U+000D CARRIAGE RETURN and U+000C FORM FEED are not included in this definition, as they are converted to U+000A LINE FEED during preprocessing.
#[inline]
pub fn is_newline(code: u8) -> bool {
    code == 0x000A || code == 0x000D || code == 0x000C
}
// A newline, U+0009 CHARACTER TABULATION, or U+0020 SPACE.
#[inline]
pub fn is_whitespace(code: u8) -> bool {
    is_newline(code) || code == 0x0009 || code == 0x0020
}
// https://drafts.csswg.org/css-syntax/#check-if-two-code-points-are-a-valid-escape
#[inline]
pub fn is_valid_escape(first: u8, second: u8) -> bool {
    first == b'\\' && !is_newline(second)
}

// https://drafts.csswg.org/css-syntax/#would-start-an-identifier
pub fn would_start_an_identifier(first: u8, second: u8, third: u8) -> bool {
    // U+002D HYPHEN-MINUS
    if first == b'-' {
        is_identifier_start(second) || second == b'-' || is_valid_escape(second, third)
    } else if is_identifier_start(first) {
        true
    } else if first == b'\\' {
        is_valid_escape(first, second)
    } else {
        false
    }
}

// https://drafts.csswg.org/css-syntax/#starts-with-a-number
pub fn would_start_a_number(first: u8, second: u8, third: u8) -> bool {
    if first == b'+' || first == b'-' {
        return is_digit(second) || (second == b'.' && is_digit(third));
    }
    if first == b'.' {
       return is_digit(second);
    }
    return is_digit(first);
}

#[inline]
pub fn utf8_is_cont_byte(byte: u8) -> bool {
    (byte & !0b0011_1111) == 0b1000_0000
}