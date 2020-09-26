use crate::tokenizer::token::Token;
use crate::tokenizer::source_location::SourceLocation;

pub struct Node<'a> {
    pub r#type: NodeType<'a>,
    pub loc: SourceLocation,
    pub children: Vec<Box<Node<'a>>>,
    leading_comments: Vec<Comment<'a>>,
    inner_comments: Vec<Comment<'a>>,
    trailing_comments: Vec<Comment<'a>>,
}

pub struct AtRule<'a> {
    name: &'a str,
    prelude: Box<Node<'a>>,
    block: Box<Node<'a>>,
}

pub struct Comment<'a> {
    pub r#value: &'a str,
    pub loc: SourceLocation,
}

pub struct Block<T> {
    children: Vec<T>,
}

pub enum NodeType<'a> {
    StyleSheet,
    AtRulePrelude,
    Function(&'a str),
    Block,
    AtRule(AtRule<'a>),
    CDC,
    CDO,
}
