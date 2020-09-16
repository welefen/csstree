// mod tokenizer;
mod tokenizer;

use std::time::{Duration, Instant};
use tokenizer::token::Token;
use tokenizer::tokenizer::Tokenizer;

// use crate::tokenizer::tokenizer;
fn main() {
    let start = Instant::now();
    let a = r#"url(  http )"#;
    let mut instance = Tokenizer::new(a);
    let mut i = 0;
    while i < 10 {
        i += 1;
        let token = instance.next();
        if let Token::EOF = token {
            break;
        }
        println!("{:?}", token);
    }
    let duration = start.elapsed();
    // println!("Time elapsed in expensive_function() is: {:?}", Byte::RIGHT_PARENTHESIS);
}
