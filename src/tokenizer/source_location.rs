#[derive(Debug)]
pub struct Position {
    pub line: u32,
    pub col: u32,
}

#[derive(Debug)]
pub struct SourceLocation {
    pub start: Position,
    pub end: Position,
}
