#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub col: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            index: 0,
            line: 1,
            col: 1,
        }
    }
}

impl Position {
    pub fn synthetic() -> Self {
        Self {
            index: 0,
            line: 0,
            col: 0,
        }
    }

    pub fn increment(&mut self) {
        self.index += 1;
        self.col += 1;
    }

    pub fn increment_line(&mut self) {
        self.line += 1;
        self.col = 0;
    }
}
