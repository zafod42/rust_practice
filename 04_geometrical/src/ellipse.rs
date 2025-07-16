

use crate::errors::ParseEllipseError;
use crate::shape::Shape;
use crate::base_types::{Point, Rect};

use std::f32::consts::PI;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Ellipse {
    pub center: Point,
    pub vradius: f32,
    pub hradius: f32,
}

impl Ellipse {
    fn new(center: Point, vradius: f32, hradius: f32) -> Ellipse {
        Ellipse {center, vradius, hradius}
    }
}

impl Shape for Ellipse {
    fn get_area(&self) -> f32 {
        PI * self.vradius * self.hradius
    }

    fn get_center(&self) -> Point {
        self.center
    }

    fn get_frame_rect(&self) -> Rect {
        Rect {
            width: self.hradius*2.0, 
            height: self.vradius*2.0, 
            pos: self.center.clone(), 
        }
    }

    fn move_at(&mut self, target: &Point) {
        self.center = target.clone();
    }

    fn move_by(&mut self, x: f32, y: f32) {
        self.center.x += x;
        self.center.y += y;
    }

    fn scale(&mut self, factor: f32) {
        self.vradius *= factor;
        self.hradius *= factor;
    }
}

impl FromStr for Ellipse {
    type Err = ParseEllipseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s
            .split_whitespace();

        let x = tokens.next().ok_or(
            ParseEllipseError {
                place: s.to_string(),
                cause: "No coords provided (need 2)".to_string(),
            })?;
        let y = tokens.next().ok_or(
            ParseEllipseError {
                place: s.to_string(),
                cause: "Not enought coords (need 2)".to_string(),
            })?;
        let vradius = tokens.next().ok_or(
            ParseEllipseError {
                place: s.to_string(),
                cause: "No vertical radius after coords".to_string(),
            })?;
        let hradius = tokens.next().ok_or(
            ParseEllipseError {
                place: s.to_string(),
                cause: "No horizontal radius after coords".to_string(),
            })?;

        let x = x.parse::<f32>().map_err(
            |_| ParseEllipseError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", x).to_string(),
            })?;
        let y = y.parse::<f32>().map_err(
            |_| ParseEllipseError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", y).to_string(),
            })?;
        let center = Point {x, y};

        let vradius = vradius.parse::<f32>().map_err(
            |_| ParseEllipseError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", vradius).to_string(),
            })?;
        let hradius = hradius.parse::<f32>().map_err(
            |_| ParseEllipseError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", hradius).to_string(),
            })?;

        if hradius <= 0.0 || vradius <= 0.0 {
            return Err(ParseEllipseError {
                place: s.to_string(),
                cause: "Invalid ellipse horisontal and vertical raidus must be greater then 0".to_string(),
            })
        }

        Ok(Ellipse::new(center, vradius, hradius))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_correct_ellipse() {
        let ellipse_str = "0.0 0.0 10.0 12.0";
        let expected = Ellipse::new(
            Point { x: 0.0, y: 0.0 },
            10.0,
            12.0,
        );

        let result: Ellipse = ellipse_str.parse().unwrap();

        assert_eq!(expected, result);
    }


    #[test]
    fn get_area_correct() {
        let ellipse = Ellipse::new(Point{x:1.0, y:2.0}, 1.0, 2.0);
        
        assert_eq!(PI * 2.0 * 1.0, ellipse.get_area());
    }

    #[test]
    fn get_frame_rect_correct() {
        let ellipse = Ellipse::new(Point{x:1.0, y:2.0}, 1.0, 2.0);
        let rect = Rect {pos:Point{x:1.0, y:2.0}, width: 4.0, height: 2.0};

        assert_eq!(rect, ellipse.get_frame_rect());
    }

    #[test]
    fn move_at_correct() {
        let mut ellipse = Ellipse::new(Point{x:1.0, y:2.0}, 1.0, 2.0);

        ellipse.move_at(&Point {x:0.0, y:0.0});

        assert_eq!(Point {x:0.0, y:0.0}, ellipse.center);
    }

    #[test]
    fn move_by_correct() {
        let mut ellipse = Ellipse::new(Point{x:1.0, y:2.0}, 1.0, 2.0);

        ellipse.move_by(-1.0, -2.0);

        assert_eq!(Point {x:0.0, y:0.0}, ellipse.center);
    }

    #[test]
    fn scale_correct() {
        let mut ellipse = Ellipse::new(Point{x:1.0, y:2.0}, 1.0, 2.0);

        ellipse.scale(2.0);

        assert_eq!(2.0, ellipse.vradius);
        assert_eq!(4.0, ellipse.hradius);
    }
}

