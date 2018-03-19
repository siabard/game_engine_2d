//! Vector 2D

use std::ops::*;

/// vector 2d
#[derive(Default, Clone, PartialEq)]
pub struct Vector2D {
    /// x
    pub x: f32,
    /// y
    pub y: f32,
}

/// impl
impl Vector2D {
    /// new
    pub fn new() -> Self {
        Default::default()
    }

    /// init
    pub fn init(x: f32, y: f32) -> Self {
        Vector2D { x: x, y: y }
    }
}

/// ADD
impl Add for Vector2D {
    type Output = Vector2D;

    /// Add
    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// +=
impl AddAssign for Vector2D {
    /// +=
    fn add_assign(&mut self, other: Vector2D) {
        self.x += other.x;
        self.y += other.y;
    }
}

/// Sub
impl Sub for Vector2D {
    type Output = Vector2D;

    /// Substract
    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// -=
impl SubAssign for Vector2D {
    /// -=
    fn sub_assign(&mut self, other: Vector2D) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

/// Multiply
impl Mul for Vector2D {
    type Output = Vector2D;

    /// Multiply
    fn mul(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

/// *=
impl MulAssign for Vector2D {
    /// *=
    fn mul_assign(&mut self, other: Vector2D) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

/// Division
impl Div for Vector2D {
    type Output = Vector2D;

    /// Division
    fn div(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

/// /=
impl DivAssign for Vector2D {
    /// /=
    fn div_assign(&mut self, other: Vector2D) {
        self.x /= other.x;
        self.y /= other.y;
    }
}
