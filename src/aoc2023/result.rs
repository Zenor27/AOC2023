use std::fmt;

pub struct AdventOfCodeMismatch {
    pub expected: u32,
    pub actual: u32,
}

impl fmt::Display for AdventOfCodeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expected {}, but got {}", self.expected, self.actual)
    }
}

pub struct AdventOfCodeOk {
    pub result: u32,
}

impl fmt::Display for AdventOfCodeOk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ok: {}", self.result)
    }
}

pub type AdventOfCodeResult = Result<AdventOfCodeOk, AdventOfCodeMismatch>;
