mod shapes;

use std::{str::FromStr, fmt::Display};

use shapes::{circle::Circle, collisions::{Collidable, Points, Contains}};

use crate::shapes::rect::Rect;

use anyhow::Result;
enum Shape {
    Rect(Rect),
    Circ(Circle),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ")
            .ok_or(anyhow::anyhow!("Invalid shape"))?;

        match shape {
            "circle" => return Ok(Shape::Circ(data.parse()?)),
            "rect" => return Ok(Shape::Rect(data.parse()?)),
            _ => return Err(anyhow::anyhow!("Invalid shape"))
        }
    }
}

impl Points for Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Rect(rect) => return rect.points(),
            Shape::Circ(circ) => return circ.points(),
        }
    }
}

impl Contains for Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Rect(rect) => return rect.contains_point(point),
            Shape::Circ(circ) => return circ.contains_point(point),
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Rect(rect) => return write!(f, "{}", rect),
            Shape::Circ(circ) => return write!(f, "{}", circ),
        }
    }
}
fn main() -> Result<()> {
    let file = std::fs::read_to_string("shapes")?;
    let shapes = file
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Shape>>();

    let collisions: Vec<(&Shape, &Shape)> = shapes
        .iter()
        .skip(1)
        .zip(shapes.iter().take(shapes.len() - 1))
        .filter(|(a, b)| a.collide(b))
        .collect();

    for (a, b) in collisions {
        println!("Collision: {} {}", a, b);
    }

    return Ok(());
}