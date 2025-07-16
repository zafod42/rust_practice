use crate::errors::ParseScaleCommandError;
use crate::base_types::Point;

pub struct Scale {
    pub origin: Point,
    pub factor: f32,
}

impl std::str::FromStr for Scale {
    type Err = ParseScaleCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut params = s.split_whitespace();
        let x = params.next().ok_or(
            ParseScaleCommandError {
                place: s.to_string(),
                cause: "No parameters provided (need 3)".to_string(),
            })?;
        let y = params.next().ok_or(
            ParseScaleCommandError {
                place: s.to_string(),
                cause: "Not enough parameters (need 3)".to_string(),
            })?;
        let factor = params.next().ok_or(
            ParseScaleCommandError {
                place: s.to_string(),
                cause: "Not enough parameters (need 3)".to_string(),
            })?;
        
        let x = x.parse::<f32>().map_err(
            |_| ParseScaleCommandError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", x).to_string(),
            })?;
        let y = y.parse::<f32>().map_err(
            |_| ParseScaleCommandError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", y).to_string(),
            })?;
        let origin= Point { x, y };

        let factor = factor.parse::<f32>().map_err (
            |_| ParseScaleCommandError {
                place: s.to_string(),
                cause: format!("Cannot parse `{}', expected float32", factor).to_string(),
            })?;

        Ok(Scale { origin, factor })
    }
}

impl Scale {
    pub fn new() -> Scale {
        Scale {
            origin: Point { x: 0.0, y: 0.0 },
            factor: 1.0,
        }
    }
}

