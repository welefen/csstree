use crate::tokenizer::tokenizer::Tokenizer;
pub struct Parser<'a> {
    input: &'a str,
    options: ParserOptions,
}

pub enum ParserContext {
    Stylesheet,
}

pub struct ParserOptions<'a> {
    context: ParserContext,
}

impl<'a> Parser<'a> {
    fn new(input: &str, options: ParserOptions) -> Parser<'a> {
        Parser { input, options }
    }
}
