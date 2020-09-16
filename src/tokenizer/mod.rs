pub mod source_location;
pub mod token;
pub mod tokenizer;

// https://drafts.csswg.org/css-syntax-3/

#[inline]
fn match_byte(byte: u8, m: &[u8]) -> bool {
    byte == (*m)[0]
}

// detect str start with BOM
fn is_bom_start(bytes: &[u8]) -> bool {
    if bytes.len() < 3 {
        return false;
    }
    let buf: &[u8] = &bytes[0..3];
    if (buf[0] == 0xef) && (buf[1] == 0xbb) && (buf[2] == 0xbf) {
        true
    } else {
        false
    }
}

// digit
// A code point between U+0030 DIGIT ZERO (0) and U+0039 DIGIT NINE (9).
pub fn is_digit(code: u8) -> bool {
    code >= 0x30 && code <= 0x39
}

// hex digit
// A digit, or a code point between U+0041 LATIN CAPITAL LETTER A (A) and U+0046 LATIN CAPITAL LETTER F (F),
// or a code point between U+0061 LATIN SMALL LETTER A (a) and U+0066 LATIN SMALL LETTER F (f).
pub fn is_hex_digit(code: u8) -> bool {
    is_digit(code) || (code >= 0x0041 && code <= 0x0046) || (code >= 0x0061 && code <= 0x0066)
}

// newline
pub fn is_newline(code: u8) -> bool {
    code == 0x000A || code == 0x000D || code == 0x000C
}

// whitespace
pub fn is_whitespace(code: u8) -> bool {
    is_newline(code) || code == 0x0020 || code == 0x0009
}
// uppercase letter
// A code point between U+0041 LATIN CAPITAL LETTER A (A) and U+005A LATIN CAPITAL LETTER Z (Z).
pub fn is_uppercase_letter(code: u8) -> bool {
    code >= 0x0041 && code <= 0x005A
}
// lowercase letter
// A code point between U+0061 LATIN SMALL LETTER A (a) and U+007A LATIN SMALL LETTER Z (z).
pub fn is_lowercase_ltter(code: u8) -> bool {
    code >= 0x0061 && code <= 0x007A
}
// letter
// An uppercase letter or a lowercase letter.
pub fn is_letter(code: u8) -> bool {
    is_uppercase_letter(code) || is_lowercase_ltter(code)
}
// non-ASCII code point
// A code point with a value equal to or greater than U+0080 <control>.
pub fn is_non_ascii(code: u8) -> bool {
    code >= 0x0080
}
// name-start code point
// A letter, a non-ASCII code point, or U+005F LOW LINE (_).
pub fn is_name_start(code: u8) -> bool {
    is_letter(code) || is_non_ascii(code) || code == 0x005F
}
// name code point
// A name-start code point, a digit, or U+002D HYPHEN-MINUS (-).
pub fn is_name(code: u8) -> bool {
    is_name_start(code) || is_digit(code) || code == 0x002D
}

// non-printable code point
// A code point between U+0000 NULL and U+0008 BACKSPACE, or U+000B LINE TABULATION,
// or a code point between U+000E SHIFT OUT and U+001F INFORMATION SEPARATOR ONE, or U+007F DELETE.
pub fn is_non_printable(code: u8) -> bool {
    code <= 0x0008 || code == 0x000B || (code >= 0x000E && code <= 0x001F) || code == 0x007F
}

// § 4.3.8. Check if two code points are a valid escape
// https://drafts.csswg.org/css-syntax/#starts-with-a-valid-escape
pub fn is_valid_escape(first: u8, second: u8) -> bool {
    // If the first code point is not U+005C REVERSE SOLIDUS (\), return false.
    if first != 0x005C {
        return false;
    }
    // Otherwise, if the second code point is a newline or EOF, return false.
    if is_newline(second) || second == 0 {
        return false;
    }
    true
}

// § 4.3.9. Check if three code points would start an identifier
// https://drafts.csswg.org/css-syntax/#would-start-an-identifier
pub fn is_identifier_start(first: u8, second: u8, third: u8) -> bool {
    // U+002D HYPHEN-MINUS
    if first == 0x002D {
        // If the second code point is a name-start code point or a U+002D HYPHEN-MINUS,
        // or the second and third code points are a valid escape, return true. Otherwise, return false.
        return is_name_start(second) || second == 0x002D || is_valid_escape(second, third);
    }
    // name-start code point
    if is_name_start(first) {
        return true;
    }
    // U+005C REVERSE SOLIDUS (\)
    if first == 0x005C {
        // If the first and second code points are a valid escape, return true. Otherwise, return false.
        return is_valid_escape(second, third);
    }
    false
}

// § 4.3.10. Check if three code points would start a number
// https://drafts.csswg.org/css-syntax/#starts-with-a-number
pub fn detect_number_start(first: u8, second: u8, third: u8) -> u8 {
    // U+002B PLUS SIGN (+)
    // U+002D HYPHEN-MINUS (-)
    if first == 0x002B || first == 0x002D {
        // If the second code point is a digit, return true.
        if is_digit(second) {
            return 2;
        }
        // Otherwise, if the second code point is a U+002E FULL STOP (.)
        // and the third code point is a digit, return true.
        // Otherwise, return false.
        if second == 0x002E || is_digit(third) {
            return 3;
        }
        return 0;
    }
    // U+002E FULL STOP (.)
    if first == 0x002E {
        // If the second code point is a digit, return true. Otherwise, return false.
        if is_digit(second) {
            return 2;
        }
        return 0;
    }
    if is_digit(first) {
        return 1;
    }
    return 0;
}
