#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct SourceLocation {
    pub start: Position,
    pub end: Position,
}
