pub mod node;
pub mod parser;
use crate::tokenizer::token::Token;

// If the current input token is a <{-token>, <[-token>, or <(-token>,
pub fn is_block_start(token: Token) -> bool {
    if let Token::LeftCurlyBracket = token {
        return true;
    }
    if let Token::LeftSquareBracket = token {
        return true;
    }
    if let Token::LeftParenthesis = token {
        return true;
    }
    false
}

// The ending token is the mirror variant of the current input token.
// (E.g. if it was called with <[-token>, the ending token is <]-token>.)
pub fn is_block_matched(start: Token, end: Token) -> bool {
    if let Token::LeftCurlyBracket = start {
        if let Token::RightCurlyBracket = end {
            return true;
        }
    }
    if let Token::LeftSquareBracket = start {
        if let Token::RightSquareBracket  = end {
            return true;
        }
    }
    if let Token::LeftParenthesis = start {
        if let Token::RightParenthesis = end {
            return true;
        }
    }
    false
}
