// mod tokenizer;
mod tokenizer;
use std::str;

use std::time::{Duration, Instant};
use tokenizer::token::Token;
use tokenizer::tokenizer::Tokenizer;

// use crate::tokenizer::tokenizer;
fn main() {
    let start = Instant::now();
    let a = "\"23af\"";
    // a.chars();
    //(189 & !0b0011_1111) == 0b1000_0000
    // let mut b = String::from(&a[0..4]);
    // println!("www{:?}", b);
    let mut instance = Tokenizer::new(a);
    let c: u8 = b'a';
    let mut i = 0;
    while i < 10 {
        i += 1;
        let token = instance.nextToken();
        if let Token::EOF = token {
            break;
        }
        println!("{:?}", token);
    }
    // let f = a.starts_with("0x61");
    // println!("{:?}", f);
    // let mut i = 0;
    // while i < 10000000 {
    //     let b = a;
    //     // b = b + "s";
    //     i += 1;
    //     // a.chars().next();
    // }

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", c);
}
