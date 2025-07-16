use std::fmt;
use std::fmt::Display;


#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub const ORIGIN: Point = Point {x:0.0, y:0.0};


impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    } 
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub pos: Point,
}

impl Rect {
    fn left_bot(&self) -> Point {
        Point {
            x: self.pos.x - self.width/2.0,
            y: self.pos.y - self.height/2.0,
        }
    }

    fn right_top(&self) -> Point {
        Point {
            x: self.pos.x + self.width/2.0,
            y: self.pos.y + self.height/2.0,
        }
    }
}


impl Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.left_bot(), self.right_top())
    }
}

