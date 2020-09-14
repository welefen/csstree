pub mod source_location;
pub mod token;
pub mod tokenizer;


// detect str start with BOM
fn has_bom(bytes: &[u8]) -> bool {
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

pub fn is_digit(code: u8) -> bool {
    code >= 0x30 && code <= 0x39
}

// newline
pub fn is_newline(code: u8) -> bool {
    code == 0x000A || code == 0x000D || code == 0x000C
}

// whitespace
pub fn is_whitespace(code: u8) -> bool {
    is_newline(code) || code == 0x0020 || code == 0x0009
}
