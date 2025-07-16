
use std::error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseScaleCommandError {
    pub place: String,
    pub cause: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseEllipseError {
    pub place: String,
    pub cause: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTriangleError {
    pub place: String,
    pub cause: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRectangleError {
    pub place: String,
    pub cause: String,
}

impl error::Error for ParseEllipseError {}
impl error::Error for ParseTriangleError {}
impl error::Error for ParseRectangleError {}
impl error::Error for ParseScaleCommandError {}

impl Display for ParseRectangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "For RECTANGLE cannot parse `{}': {}", self.place, self.cause)
    }
}

impl Display for ParseTriangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "For TRIANGLE cannot parse `{}': {}", self.place, self.cause)
    }
}

impl Display for ParseEllipseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "For ELLIPSE cannot parse `{}': {}", self.place, self.cause)
    }
}

impl Display for ParseScaleCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "For SCALE cannot parse `{}': {}", self.place, self.cause)
    }
}
