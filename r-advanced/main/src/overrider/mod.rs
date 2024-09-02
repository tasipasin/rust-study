use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// Add trait code:
// A trait with one method and an associated type.
// The Rhs (short for Right Hand Side) part is a syntax called default type Parameter.
// It defines the type of the rhs parameter in the add method.
// If not defined, by default it will be the same as the type implementing Add

// trait Add<Rhs=Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

impl Add for Point {
    // Associated Type
    type Output = Point;
    // Overloads the + operator
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Adds a point with a signed 32-bits integer for both position values
// The Rhs is defined as a i32
impl Add<i32> for Point {
    // Associated Type
    type Output = Point;
    // Overloads the + operator
    fn add(self, amount: i32) -> Point {
        Point {
            x: self.x + amount,
            y: self.y + amount,
        }
    }
}

// use crate::depender::PrintBox;

// It fails, since Point does not implements Display trait
// impl PrintBox for Point{}
