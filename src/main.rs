// mod tokenizer;
mod tokenizer;

use tokenizer::token::Token;
use tokenizer::tokenizer::Tokenizer;

// use crate::tokenizer::tokenizer;
fn main() {
    let a = "   {";
    let mut instance = Tokenizer::new(a);
    loop {
        let token = instance.next();
        if let Token::EOF = token {
            break;
        }
        println!("{:?}", token);
    }
}
