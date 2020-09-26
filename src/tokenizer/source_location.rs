#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct SourceLocation {
    pub start: Position,
    pub end: Position,
}
