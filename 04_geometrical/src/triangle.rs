
use std::str::FromStr;

use crate::errors::ParseTriangleError;
use crate::shape::Shape;
use crate::base_types::{Point, Rect, ORIGIN};

#[derive(Debug, PartialEq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {

    fn new(a: Point, b: Point, c: Point) -> Triangle {
        Triangle { a, b, c }
    }

    fn center(&self) -> Point {
        let mut center: Point = self.a.clone() + self.b.clone() + self.c.clone();
        center.x /= 3.0;
        center.y /= 3.0;
        center.clone()
    }

    fn scale_points(&mut self, factor: f32) {
        self.a *= factor;
        self.b *= factor;
        self.c *= factor;
    }
     fn is_valid(&self) -> bool {
         if self.a == self.b || self.b == self.c || self.a == self.c {
             return false;
         }
         let ab = Point { x: self.b.x - self.a.x, y: self.b.y - self.a.y };
         let ac = Point { x: self.c.x - self.a.x, y: self.c.y - self.a.y };
         let cross_product = ab.x * ac.y - ab.y * ac.x;
         const EPS: f32 = 1e-6;
         if cross_product.abs() < EPS {
             return false;
         }
         true
     }
}

impl Shape for Triangle {

    fn get_center(&self) -> Point {
        self.center()
    }

    fn get_area(&self) -> f32 {
        let ab = self.b.clone() - self.a.clone();
        let ac = self.c.clone() - self.a.clone();

        let vec_mul: f32 = (ab.x * ac.y - ab.y * ac.x).abs();
        let area = 0.5 * vec_mul;
        area
    }
    fn get_frame_rect(&self) -> Rect {
        let x_coords = vec![self.a.x, self.b.x, self.c.x];
        let y_coords = vec![self.a.y, self.b.y, self.c.y];

        let mut x = f32::MAX;
        let mut x_max = f32::MIN;
        for val in x_coords.into_iter() {
            if x > val {
                x = val;
            }
            if x_max < val {
                x_max = val;
            }
        }

        let mut y = f32::MAX;
        let mut y_max = f32::MIN;
        for val in y_coords.into_iter() {
            if y > val {
                y = val;
            }
            if y_max < val {
                y_max = val;
            }
        }

        let left_bot = Point { x, y };

        let width = x_max - x;
        let height = y_max - y;
    
        let pos = Point { 
            x: x + width /2.0,
            y: y + height /2.0,
        };

        Rect{
            pos,
            width,
            height,
        }
    }
    fn move_at(&mut self, target: &Point) {
        let delta = *target - self.center();
        self.move_by(delta.x, delta.y);
    }

    fn move_by(&mut self, x: f32, y: f32) {
        self.a.x += x;
        self.a.y += y;

        self.b.x += x;
        self.b.y += y;

        self.c.x += x;
        self.c.y += y;

    }
    fn scale(&mut self, factor: f32) {
        let center = self.center();

        self.move_at(&ORIGIN);

        self.scale_points(factor);

        self.move_at(&center);
    }
}


impl FromStr for Triangle {
    type Err = ParseTriangleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> { 

        let mut tokens = s
            .split_whitespace();

        let x1 = tokens.next().ok_or(
            ParseTriangleError {
                place: s.to_string(),
                cause: "No coords provided (need 6)".to_string(),
            })?;
        let y1 = tokens.next().ok_or(
            ParseTriangleError {
                place: s.to_string(),
                cause: "Not enough coords (need 6)".to_string(),
            })?;
        let x2 = tokens.next().ok_or(
            ParseTriangleError {
                place: s.to_string(),
                cause: "Not enough coords (need 6)".to_string(),
            })?;
        let y2 = tokens.next().ok_or(
            ParseTriangleError {
                place: s.to_string(),
                cause: "Not enough coords (need 6)".to_string(),
            })?;
        let x3 = tokens.next().ok_or(
            ParseTriangleError {
                place: s.to_string(),
                cause: "Not enough coords (need 6)".to_string(),
            })?;
        let y3 = tokens.next().ok_or(
            ParseTriangleError {
                place: s.to_string(),
                cause: "Not enough coords (need 6)".to_string(),
            })?;
        let x = x1.parse::<f32>().map_err(
            |_| ParseTriangleError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", x1).to_string(),
            })?;
        let y = y1.parse::<f32>().map_err(
            |_| ParseTriangleError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", y1).to_string(),
            })?;
        let a = Point { x, y };


        let x = x2.parse::<f32>().map_err(
            |_| ParseTriangleError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", x2).to_string(),
            })?;
        let y = y2.parse::<f32>().map_err(
            |_| ParseTriangleError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", y2).to_string(),
            })?;
        let b = Point { x, y };

        let x = x3.parse::<f32>().map_err(
            |_| ParseTriangleError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", x3).to_string(),
            })?;
        let y = y3.parse::<f32>().map_err(
            |_| ParseTriangleError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", y3).to_string(),
            })?;
        let c = Point { x, y };
        
        let tri = Triangle::new(a, b, c);
        if !tri.is_valid() {
            return Err(ParseTriangleError {
                place: s.to_string(),
                cause: "Invalid triangle".to_string(),
            })
        }

        Ok(Triangle::new(a, b, c))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_correct_triangle() {
        let triangle_str = "0.0 0.0 1.0 1.0 1.0 0.0";
        let expected = Triangle::new(
            Point {x:0.0, y:0.0},
            Point {x:1.0, y:1.0},
            Point {x:1.0, y:0.0},
        );

        let result: Triangle = triangle_str.parse().unwrap();

        assert_eq!(expected, result);
    }


    #[test]
    fn center_correct() {
        let triangle = Triangle::new(
            Point {x:1.0, y:1.0},
            Point {x:6.0, y:1.0},
            Point {x:5.0, y:5.0},
        );

        assert_eq!(Point {x:4.0, y: 7.0/3.0}, triangle.center());
    }

    #[test]
    fn get_area_correct() {
        let triangle = Triangle::new(
            Point {x:2.0, y:3.0},
            Point {x:5.0, y:7.0},
            Point {x:1.0, y:10.0},
        );

        assert_eq!(12.5, triangle.get_area());
    }

    #[test]
    fn move_by_correct() {
        let mut triangle = Triangle::new(
            Point {x:2.0, y:3.0},
            Point {x:5.0, y:7.0},
            Point {x:1.0, y:10.0},
        );

        let expected = Triangle::new(
            Point {x:4.0, y:5.0},
            Point {x:7.0, y:9.0},
            Point {x:3.0, y:12.0},
        );

        triangle.move_by(2.0, 2.0);


        assert_eq!(expected.a, triangle.a);
        assert_eq!(expected.b, triangle.b);
        assert_eq!(expected.c, triangle.c);
    }

    #[test]
    fn move_at_correct() {
        let mut triangle = Triangle::new(
            Point {x:0.0, y:0.0},
            Point {x:0.0, y:3.0},
            Point {x:3.0, y:0.0}
        );

        let expected = Triangle::new(
            Point {x:1.0, y:1.0},
            Point {x:1.0, y:4.0},
            Point {x:4.0, y:1.0}
        );

        triangle.move_at( &Point { x:2.0, y:2.0});

        assert_eq!(expected.a, triangle.a);
        assert_eq!(expected.b, triangle.b);
        assert_eq!(expected.c, triangle.c);
    }

    #[test]
    fn scale_correct() {
        let mut triangle = Triangle::new(
            Point { x:4.0, y:3.0 },
            Point { x:6.0, y:3.0 },
            Point { x:4.0, y:5.0 },
        );

        let expected = Triangle::new(
            Point { x:3.0, y:2.0 },
            Point { x:3.0, y:7.0 },
            Point { x:8.0, y:2.0 },
        );

        triangle.scale(2.5);

        let _ = relative_eq!(expected.a.x, triangle.a.x);
        let _ = relative_eq!(expected.a.y, triangle.a.y);
        let _ = relative_eq!(expected.b.x, triangle.b.x);
        let _ = relative_eq!(expected.b.y, triangle.b.y);
        let _ = relative_eq!(expected.c.x, triangle.c.x);
        let _ = relative_eq!(expected.c.y, triangle.c.y);
    }

}
