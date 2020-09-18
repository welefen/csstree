#[derive(Debug)]
pub enum Token<'a> {
    EOF,                   // <EOF-token>
    Ident(&'a str),        // <ident-token>
    Function(&'a str),     // <function-token>
    AtKeyword(&'a str),    // <at-keyword-token>
    Hash(&'a str),         // <hash-token>
    String(&'a str),       // <string-token>
    BadString(&'a str),    // <bad-string-token>
    Url(&'a str),          // <url-token>
    BadUrl(&'a str),       // <bad-url-token>
    Delim(&'a str),        // <delim-token>
    Number(&'a str),       // <number-token>
    Percentage(&'a str),   // <percentage-token>
    Dimension(&'a str),    // <dimension-token>
    WhiteSpace(&'a str),   // <whitespace-token>
    CDO,                   // <CDO-token>
    CDC,                   // <CDC-token>
    Colon,                 // <colon-token>      :
    Semicolon,             // <semicolon-token> ;
    Comma,                 // <comma-token>     ,
    LeftSquareBracket,     // <[-token>
    RightSquareBracket,    // <]-token>
    LeftParenthesis,       // <(-token>
    RightParenthesis,      // <)-token>
    LeftCurlyBracket,      // <{-token>
    RightCurlyBracket,     // <}-token>
    Comment(&'a str),      // <comment-token>
}