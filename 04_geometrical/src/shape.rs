use std::fmt;


use crate::base_types::{Point, Rect};


///
/// # Shape abstract class aka Interface aka Trait
///
///
pub trait Shape: fmt::Debug {
    fn get_area(&self) -> f32;
    fn get_frame_rect(&self) -> Rect;
    fn move_at(&mut self, target: &Point);
    fn move_by(&mut self, x: f32, y: f32);
    fn scale(&mut self, factor: f32);
    fn get_center(&self) -> Point;
}
