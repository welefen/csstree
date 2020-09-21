mod tokenizer;
use std::fs;

use std::time::Instant;
use tokenizer::token::Token;
use tokenizer::tokenizer::Tokenizer;



fn token_by_csstree(content: &str) {
    let mut instance = Tokenizer::new(content);
    // let mut arr = vec![];
    loop {
        let token = instance.next_token();
        if let Token::EOF = token {
            break;
        } else {
            // arr.push(token);
        }
        // println!("{:?}", token);
    }
}

// use cssparser::Tokenizer;
// fn token_by_cssparser(content: &str) {
//     let mut instance = Tokenizer::new(content);
//     loop {
//         let token = instance.next();
//         match token {
//             Ok(t) => (),
//             Err(e) => break
//         }
//     }
// }

fn main() {
    let content = fs::read_to_string("./files/1.css").expect("");
    // content.chars();
    // let content = r"a a\622222  wwww";
    let start = Instant::now();
    token_by_csstree(&content);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
