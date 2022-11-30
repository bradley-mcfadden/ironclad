use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

pub const UP: Vec2 = Vec2 { x: 0, y: -1 };
pub const DOWN: Vec2 = Vec2 { x: 0, y: 1 };
pub const LEFT: Vec2 = Vec2 { x: -1, y: 0 };
pub const RIGHT: Vec2 = Vec2 { x: 1, y: 0 };

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

impl Vec2 {
    pub fn new(xi: i32, yi: i32) -> Vec2 {
        Vec2{x: xi, y: yi}
    }

    pub fn left(self) -> Vec2 {
        self + LEFT
    }

    pub fn right(self) -> Vec2 {
        self + RIGHT
    }

    pub fn up(self) -> Vec2 {
        self + UP
    }

    pub fn down(self) -> Vec2 {
        self + DOWN
    }

    pub fn scale(self, factor: i32) -> Vec2 {
        Vec2::new(self.x * factor, self.y * factor)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Display for Vec2 {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(fmt, "Vec2 x:{} y:{}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::*;

    #[test]
    fn vec_add() {
        assert_eq!(
            Vec2::new(2, 2) + Vec2::new(-1, 1),
            Vec2::new(1, 3)
        )
    }

    #[test]
    fn vec_sub() {
        assert_eq!(
            Vec2::new(2, 2) - Vec2::new(-1, 1),
            Vec2::new(3, 1)
        )
    }

    #[test]
    fn vec_up() {
        let v = Vec2::new(2, 2);
        assert_eq!(v.up(), v + UP)
    }

    #[test]
    fn vec_down() {
        let v = Vec2::new(2, 2);
        assert_eq!(v.down(), v + DOWN)
    }

    #[test]
    fn vec_left() {
        let v = Vec2::new(2, 2);
        assert_eq!(v.left(), v + LEFT)
    }

    #[test]
    fn vec_right() {
        let v = Vec2::new(2, 2);
        assert_eq!(v.right(), v + RIGHT)
    }

    #[test]
    fn scale() {
        let v = Vec2::new(1, 1);
        assert_eq!(v.scale(2), Vec2::new(2, 2));
        assert_eq!(v.scale(0), Vec2::new(0, 0));
    }
}
