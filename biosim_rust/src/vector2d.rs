use num::Num;

use std::ops::{Add, Sub, Mul};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> 
    where T: Num {
    pub fn new(x: T, y: T) -> Self {
        Self {x,y}
    }
}


impl<T> Add for Vector2D<T> 
    where T: Num {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}


impl<T> Sub for Vector2D<T>
    where T: Num {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Dot product for vector
impl<T> Mul for Vector2D<T>
    where T: Num {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vector2d() {
        let a = Vector2D::new(2,5);
        let b = Vector2D::new(5,6);

        assert_eq!(a+b, Vector2D::new(7,11));
    }

    #[test]
    fn substract_vector2d() {
        let a = Vector2D::new(4, 20);
        let b = Vector2D::new(8, 11);

        assert_eq!(a-b, Vector2D::new(-4, 9));
    }

    #[test]
    fn dot_product_vector2d() {
        let a = Vector2D::new(5,6);
        let b = Vector2D::new(10,2);

        assert_eq!(a*b, 62);
    }
}
