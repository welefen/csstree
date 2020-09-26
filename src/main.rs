mod tokenizer;
mod parser;

use std::time::Instant;
// use tokenizer::token::Token;
use parser::parser::Parser;
use parser::parser::ParserOptions;
use parser::parser::ParserContext;


// fn token_by_csstree(content: &str) {
//     let mut instance = Tokenizer::new(content);
//     // let mut arr = vec![];
//     loop {
//         let token = instance.next_token();
//         if let Token::EOF = token {
//             break;
//         } else {
//             // arr.push(token);
//         }
//         // println!("{:?}", token);
//     }
// }

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
    let content = r#"@font-face {
    font-family: quot\e;
    src: local('SimSun');    
    unicode-range: U+0025-00FF, U+4??; 
    }
    "#;
    // let content = fs::read_to_string("./files/1.css").expect("");
    let start = Instant::now();
    // token_by_csstree(&content);
    let options = ParserOptions {
        context: ParserContext::Stylesheet
    };
    let mut instance = Parser::new(&content, options);

    // let mut a = vec![1];
    // println!("{:?}", a.len());

    // let s = content.chars().count();
    // let mut a = content.chars();
    // loop {
    //     let s = a.next();
    //     match s {
    //         Some(w) => {},
    //         None => break
    //     }
    // };

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
