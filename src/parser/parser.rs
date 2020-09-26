use super::{is_block_matched, is_block_start};
use crate::parser::node::{Comment, Node, NodeType};
use crate::tokenizer::source_location::{Position, SourceLocation};
use crate::tokenizer::token::Token;
use crate::tokenizer::tokenizer::Tokenizer;

struct TokenContext<'a> {
    token: Token<'a>,
    loc: SourceLocation,
    comments: Vec<Comment<'a>>,
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    options: ParserOptions,
}

pub enum ParserContext {
    Stylesheet,
}

pub struct ParserOptions {
    pub context: ParserContext,
}

impl<'a> Parser<'a> {
    pub fn new(input: &str, options: ParserOptions) -> Parser {
        Parser {
            tokenizer: Tokenizer::new(input),
            options,
        }
    }
    fn get_token_and_loc(&mut self) -> (Token<'a>, SourceLocation) {
        let start = Position {
            line: self.tokenizer.line,
            column: self.tokenizer.column,
        };
        let token = self.tokenizer.next_token();
        let end = Position {
            line: self.tokenizer.line,
            column: self.tokenizer.column,
        };
        let loc = SourceLocation { start, end };
        (token, loc)
    }
    fn consume_token(&mut self) -> TokenContext<'a> {
        let mut comments: Vec<Comment<'a>> = vec![];
        loop {
            let (token, loc) = self.get_token_and_loc();
            if let Token::Comment(v) = token {
                let comment = Comment { r#value: v, loc };
                comments.push(comment);
                continue;
            }
            return TokenContext {
                token,
                loc,
                comments,
            };
        }
    }
    // pub fn parse(&self) {
    //     self.parse_stylesheet()
    // }

    // fn parse_stylesheet(&mut self) {
    //     self.parse_list_rules(true)
    // }
    // fn parse_list_rules(&self, top_level: bool) -> Vec<Node<'a>> {
    //     let nodes: Vec<Node<'a>> = Vec::with_capacity(1);
    //     loop {
    //         let token = self.token();
    //         let r = match token {
    //             Token::WhiteSpace(_) => continue,
    //             Token::EOF => break,
    //             Token::CDC | Token::CDO => {
    //                 if top_level {
    //                     continue;
    //                 } else {
    //                     self.parse_qualified_rule(token)
    //                 }
    //             }
    //             Token::AtKeyword(_) => self.parse_at_rule(token),
    //             _ => self.parse_qualified_rule(token),
    //         };
    //     }
    //     nodes
    // }
    // // https://drafts.csswg.org/css-syntax-3/#qualified-rule-diagram
    // fn parse_qualified_rule(&self) -> Node<'a> {}
    // fn parse_at_rule(&self) {}
    // // https://drafts.csswg.org/css-syntax-3/#consume-component-value
    // fn consume_component_value(&self) {
    //     let token = self.token();
    //     if is_block_start(token) {
    //         return self.consume_simpile_block();
    //     }
    //     if let Token::Function(name) = token {
    //         return self.consume_function(name);
    //     }
    // }
    // fn createNode(token: Token<'a>, r#type: NodeType) -> Node {}
    // // https://drafts.csswg.org/css-syntax-3/#consume-a-function
    // fn consume_function(&self, name: &'a str) -> Node<'a> {
    //     let node = Node {
    //         r#type: NodeType::Function(name),
    //     };
    // }
    // // https://drafts.csswg.org/css-syntax-3/#consume-simple-block
    // fn consume_simpile_block(&self, start: Token<'a>) {
    //     let nodes: Vec<Node<'a>> = Vec::with_capacity(1);
    //     loop {
    //         let token = self.token();
    //         if token == Token::EOF {
    //             return nodes;
    //         }
    //         if is_block_matched(start, token) {
    //             return nodes;
    //         }
    //         self.consume_component_value(token);
    //     }
    // }
}
