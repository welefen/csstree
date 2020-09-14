// mod tokenizer;
mod tokenizer;

use tokenizer::tokenizer::Tokenizer;

// use crate::tokenizer::tokenizer;
fn main () {
    let a = "{";
    let mut instance = Tokenizer::new(a);
    let token = instance.next();
    println!("{:?}", token);
}