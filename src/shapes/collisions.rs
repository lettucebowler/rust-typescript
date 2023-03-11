pub trait Collidable<T> {
    fn collide(&self, other: &T) -> bool;
    fn collides(&self, others: &[T]) -> bool {
        for other in others {
            if self.collide(other) {
                return true;
            }
        }
        return false;
        
    }

}
pub struct PointIter {
    pub points: Vec<(f64, f64)>,
    pub idx: usize,
}

impl From<Vec<(f64, f64)>> for PointIter {
    fn from(value: Vec<(f64, f64)>) -> Self {
        return PointIter {
            points: value,
            idx: 0,
        };
    }
}

impl Iterator for PointIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.points.len() {
            return None;
        }

        let point = self.points[self.idx];
        self.idx += 1;

        return Some(point);
    }
}

pub trait Points {
    fn points(&self) -> PointIter;
}

pub trait Contains {
    fn contains_point(&self, point: (f64, f64)) -> bool;
}

impl<T, V> Collidable<T> for V
where T: Points,
      V: Contains
{
    fn collide(&self, other: &T) -> bool {
        for point in other.points() {
            if self.contains_point(point) {
                return true;
            }
        }

        return false;
    }
}