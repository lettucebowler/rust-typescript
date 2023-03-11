use std::{fmt::Display, str::FromStr};

use super::{area::Area, collisions::{Contains, Points}};

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Points for Rect {
    fn points(&self) -> super::collisions::PointIter {
        return super::collisions::PointIter {
            points: vec![
                (self.x, self.y),
                (self.x + self.width, self.y),
                (self.x, self.y + self.height),
                (self.x + self.width, self.y + self.height),
            ],
            idx: 0,
        };
    }
}

impl Contains for Rect {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return x >= self.x && x <= self.x + self.width &&
            y >= self.y && y <= self.y + self.height;
    }
}

impl Default for Rect {
    fn default() -> Self {
        return Rect {
            x: 0.0,
            y: 0.0,
            height: 10.0,
            width: 10.0,
        }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Rectangle({}, {}) : {}x{}", self.x, self.y, self.width, self.height);
    }
}

impl FromStr for Rect {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 4 {
            return Err(anyhow::anyhow!("Invalid number of parts"));
        }

        return Ok(Rect {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            width: parts[2].parse()?,
            height: parts[3].parse()?,
        });
    }
}
