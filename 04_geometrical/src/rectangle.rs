use std::str::FromStr;

use crate::errors::ParseRectangleError;
use crate::shape::Shape;
use crate::base_types::{Point, Rect, ORIGIN};

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub left_bot: Point,
    pub right_top: Point,
}

impl Rectangle {
    fn new(left_bot: Point, right_top: Point) -> Rectangle {
        Rectangle {
            left_bot,
            right_top,
        }
    }

    fn width(&self) -> f32 {
        let first = &self.left_bot;
        let second = &self.right_top;

        second.x - first.x
    }

    fn height(&self) -> f32 {
        let first = &self.left_bot;
        let second = &self.right_top;

        second.y - first.y
    }

    fn center(&self) -> Point {
        let width = self.width();
        let height = self.height();

        let center = &self.left_bot;

        let x = center.x + width/2.0;
        let y = center.y + height/2.0;

        Point {
            x,
            y,
        }
    }

    fn scale_points(&mut self, scale: f32) {
        self.left_bot *= scale;
        self.right_top *= scale;
    }
}

impl Shape for Rectangle {
    
    fn get_center(&self) -> Point {
        self.center()
    }

    fn get_area(&self) -> f32 { 
        self.width() * self.height()
    }

    fn get_frame_rect(&self) -> Rect { 
        Rect {
            width: self.width(),
            height: self.height(),
            pos: self.center(),
        }
    }

    /// # Передвижение в конкретную точку
    ///
    /// По заданию у функции должно быть два варината, то есть
    /// нужно перегрузить функцию, но перегрузка функций в Rust
    /// не доступна. Значит надо делать две разных функции -- move_at и move_by
    ///
    /// (см. [move_by](#method.move_by))
    ///
    fn move_at(&mut self, target: &Point) { 
        let width = self.width();
        let height = self.height();
    
        let new_bot = Point {
            x: target.x - width/2.0,
            y: target.y - height/2.0,
        };

        let new_top = Point {
            x: target.x + width/2.0,
            y: target.y + height/2.0,
        };

        self.left_bot = new_bot.clone();
        self.right_top = new_top.clone();
    }

    /// # Передвижение по смещению
    ///
    /// По заданию у функции должно быть два варината, то есть
    /// нужно перегрузить функцию, но перегрузка функций в Rust
    /// не доступна. Значит надо делать две разных функции -- move_at и move_by
    ///
    /// (см. [move_at](#method.move_at))
    ///
    fn move_by(&mut self, x: f32, y: f32) {
        self.left_bot.x += x;
        self.right_top.x += x;

        self.left_bot.y += y;
        self.right_top.y += y;
    }

    fn scale(&mut self, factor: f32) {
        let center = self.center();
        
        self.move_at(&ORIGIN);

        self.scale_points(factor);

        self.move_at(&center);
    }
}

impl FromStr for Rectangle {
    type Err = ParseRectangleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s
            .split_whitespace();

        let x1 = tokens.next().ok_or(
                ParseRectangleError {
                    place: s.to_string(), 
                    cause: "No coords provided (need 4)".to_string(),
                })?;

        let y1 = tokens.next().ok_or(
                ParseRectangleError {
                    place: s.to_string(), 
                    cause: "Not enought coords (need 4)".to_string(),
                })?;

        let x2 = tokens.next().ok_or(
                ParseRectangleError {
                    place: s.to_string(), 
                    cause: "Not enought coords (need 4)".to_string(),
                })?;

        let y2 = tokens.next().ok_or(
                ParseRectangleError {
                    place: s.to_string(), 
                    cause: "Not enought coords (need 4)".to_string(),
                })?;

        let x = x1.parse::<f32>().map_err(
            |_| ParseRectangleError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", x1).to_string(),
            })?;
        let y = y1.parse::<f32>().map_err(
            |_| ParseRectangleError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", y1).to_string(),
            })?;
        let left_bot = Point {x, y};

        let x = x2.parse::<f32>().map_err(
            |_| ParseRectangleError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", x2).to_string(),
            })?;
        let y = y2.parse::<f32>().map_err(
            |_| ParseRectangleError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", y2).to_string(),
            })?;
        let right_top = Point {x, y};

        if right_top.x < left_bot.x || right_top.y < left_bot.y {
            return Err(ParseRectangleError {
                place: s.to_string(),
                cause: "right top point coords must be greater than left bot point".to_string(),
            })
        }
        if right_top == left_bot {
            return Err(ParseRectangleError {
                place: s.to_string(),
                cause: "corners must be different".to_string(),
            })
        }

        Ok(Rectangle::new(left_bot, right_top))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_correct_rectangle() {
        let rectangle_str = "-5.9 -3.4 3.0 4.0";
        let expected = Rectangle::new(
            Point { x: -5.9, y: -3.4 },
            Point { x: 3.0, y: 4.0 },
        );

        let result: Rectangle = rectangle_str.parse().expect("Cannot parse rectangle");

        assert_eq!(expected, result);
    }

    #[test]
    fn get_frame_rect_correct_rect() {
        let rectangle = Rectangle {
            left_bot: Point { x: -1.0, y: -1.0},
            right_top: Point { x: 2.0, y: 5.0},
        };

        assert_eq!(
            Rect {
                pos: Point {x: 0.5, y: 2.0}, 
                width: 3.0, 
                height: 6.0
            }, 
            rectangle.get_frame_rect()
        );
    }

    #[test]
    fn move_at_correct_move() {
        let mut rectangle = Rectangle {
            left_bot: Point { x: -1.0, y: -1.0},
            right_top: Point { x: 2.0, y: 5.0},
        };

        rectangle.move_at(&Point {x:4.0, y:3.0});
        
        assert_eq!(Point {x:2.5, y:0.0}, rectangle.left_bot);
        assert_eq!(Point {x:5.5, y:6.0}, rectangle.right_top);

    }

    #[test]
    fn move_by_correct_move() {
        let mut rectangle = Rectangle {
            left_bot: Point { x: -1.0, y: -1.0},
            right_top: Point { x: 2.0, y: 5.0},
        };

        rectangle.move_by(3.5, 1.0);
        
        assert_eq!(Point {x:2.5, y:0.0}, rectangle.left_bot);
        assert_eq!(Point {x:5.5, y:6.0}, rectangle.right_top);
    }

    #[test]
    fn width_correct_width() {
        let rectangle = Rectangle {
            left_bot: Point { x: -1.0, y: -1.0},
            right_top: Point { x: 2.0, y: 5.0},
        };

        assert_eq!(
            3.0,
            rectangle.width()
        );
    }

    #[test]
    fn height_correct_height() {
        let rectangle = Rectangle {
            left_bot: Point { x: -1.0, y: -1.0},
            right_top: Point { x: 2.0, y: 5.0},
        };

        assert_eq!(
            6.0,
            rectangle.height()
        );
    }

    #[test]
    fn scale_correct() {
        let mut rectangle = Rectangle {
            left_bot: Point { x: 2.0, y: 2.0},
            right_top: Point { x: 6.0, y: 5.0},
        };

        rectangle.scale(2.0);

        assert_eq!(Point { x: 0.0, y: 0.5}, rectangle.left_bot);
        assert_eq!(Point { x: 8.0, y: 6.5}, rectangle.right_top);
    }
}
