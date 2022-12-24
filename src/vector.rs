use crate::Real;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector<T, const LEN: usize>
where
    [T; LEN]:,
{
    elements: [T; LEN],
}

impl<T: Default + Copy, const LEN: usize> Default for Vector<T, { LEN }> {
    fn default() -> Self {
        Self {
            elements: [T::default(); LEN],
        }
    }
}

impl<T: Copy + Neg<Output = T>, const LEN: usize> Vector<T, { LEN }> {
    pub fn inverse(&self) -> Self {
        let mut elements: [T; LEN] = self.elements;
        elements.iter_mut().for_each(|a| *a = -*a);
        Self { elements }
    }
}

impl<T, const LEN: usize> Index<usize> for Vector<T, { LEN }> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T, const LEN: usize> IndexMut<usize> for Vector<T, { LEN }> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<T: Copy + Mul<T, Output = T>, const LEN: usize> Mul<T> for Vector<T, { LEN }> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let mut elements: [T; LEN] = self.elements;
        elements.iter_mut().for_each(|a| *a = *a * rhs);
        Self { elements }
    }
}

impl<T: Copy + Mul<T, Output = T>, const LEN: usize> Mul<Vector<T, { LEN }>>
    for Vector<T, { LEN }>
{
    type Output = Self;

    fn mul(self, rhs: Vector<T, { LEN }>) -> Self::Output {
        let mut elements: [T; LEN] = self.elements;
        elements
            .iter_mut()
            .zip(rhs.elements.iter())
            .for_each(|(a, b)| *a = *a * *b);
        Self { elements }
    }
}

impl<T: Mul<Output = T> + Copy, const LEN: usize> MulAssign for Vector<T, { LEN }> {
    fn mul_assign(&mut self, rhs: Self) {
        self.elements
            .iter_mut()
            .zip(rhs.elements.iter())
            .for_each(|(a, b)| *a = *a * *b);
    }
}

impl<T: Copy + Mul<T, Output = T>, const LEN: usize> MulAssign<T> for Vector<T, { LEN }> {
    fn mul_assign(&mut self, rhs: T) {
        self.elements.iter_mut().for_each(|a| *a = *a * rhs);
    }
}

impl<T: Add<Output = T> + Copy, const LEN: usize> Add for Vector<T, { LEN }> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut elements: [T; LEN] = self.elements;
        elements
            .iter_mut()
            .zip(rhs.elements.iter())
            .for_each(|(a, b)| *a = *a + *b);
        Self { elements }
    }
}

impl<T: Add<Output = T> + Copy, const LEN: usize> AddAssign for Vector<T, { LEN }> {
    fn add_assign(&mut self, rhs: Self) {
        self.elements
            .iter_mut()
            .zip(rhs.elements.iter())
            .for_each(|(a, b)| *a = *a + *b);
    }
}

impl<T: Sub<Output = T> + Copy, const LEN: usize> Sub for Vector<T, { LEN }> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut elements: [T; LEN] = self.elements;
        elements
            .iter_mut()
            .zip(rhs.elements.iter())
            .for_each(|(a, b)| *a = *a - *b);
        Self { elements }
    }
}

impl<const LEN: usize> Vector<Real, { LEN }> {
    pub fn magnitude(&self) -> Real {
        self.magnitude_squared().sqrt()
    }

    pub fn magnitude_squared(&self) -> Real {
        self.elements.iter().fold(0 as _, |acc, e| acc + e.powi(2))
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn normalize(&self) -> Self {
        let length = self.magnitude();
        if length > 0.0 {
            *self * length.recip()
        } else {
            *self
        }
    }

    pub fn dot(&self, rhs: &Self) -> Real {
        self.elements
            .iter()
            .zip(rhs.elements.iter())
            .fold(0 as Real, |acc, (a, b)| (*a * *b) + acc)
    }
}

pub type Vector3 = Vector<Real, 3>;

impl Vector3 {
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self {
            elements: [x, y, z],
        }
    }

    pub fn x_axis() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn y_axis() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn z_axis() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn x(&self) -> Real {
        self[0]
    }

    pub fn y(&self) -> Real {
        self[1]
    }

    pub fn z(&self) -> Real {
        self[2]
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn dimensions() {
        let (x, y, z) = (1.0, 2.0, 3.0);
        let vector = Vector3::new(x, y, z);
        assert_eq!(vector.x(), x);
        assert_eq!(vector.y(), y);
        assert_eq!(vector.z(), z);
    }

    #[test]
    pub fn inverse() {
        let (x, y, z) = (1.0, 2.0, 3.0);
        assert_eq!(
            Vector3::new(x, y, z).inverse(),
            Vector3 {
                elements: [-x, -y, -z]
            }
        );
    }

    #[test]
    pub fn magnitude() {
        let (x, y, z) = (1.0, 2.0, 3.0);
        let magnitude_squared = x * x + y * y + z * z;
        assert_eq!(Vector3::new(x, y, z).magnitude_squared(), magnitude_squared);
        assert_eq!(Vector3::new(x, y, z).magnitude(), magnitude_squared.sqrt());
    }

    #[test]
    pub fn normalize() {
        let (x, y, z) = (1.0, 2.0, 3.0);
        let magnitude = ((x * x + y * y + z * z) as f32).sqrt();
        assert_eq!(
            Vector3::new(x, y, z).normalize(),
            Vector3::new(x / magnitude, y / magnitude, z / magnitude)
        );
    }

    #[test]
    pub fn add() {
        assert_eq!(
            Vector3::new(1.0, 2.0, 3.0) + Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(2.0, 4.0, 6.0)
        );
    }

    #[test]
    pub fn add_assign() {
        let mut vector = Vector3::new(1.0, 2.0, 3.0);
        vector += Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(vector, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    pub fn sub() {
        assert_eq!(
            Vector3::new(2.0, 4.0, 6.0) - Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    pub fn multiply_scalar() {
        assert_eq!(
            Vector3::new(1.0, 2.0, 3.0) * 2.0,
            Vector3::new(2.0, 4.0, 6.0)
        );
    }

    #[test]
    pub fn index() {
        assert_eq!(Vector3::new(1.0, 2.0, 3.0)[1], 2.0);
    }

    #[test]
    pub fn index_mut() {
        let mut vector = Vector3::new(1.0, 2.0, 3.0);
        vector[1] = 0.0;
        assert_eq!(vector[1], 0.0);
    }

    #[test]
    pub fn dot_product() {
        let dot_product = Vector3::new(1.0, 2.0, 3.0).dot(&Vector3::new(3.0, 2.0, 1.0));
        assert_eq!(dot_product, 10.0);
    }

    #[test]
    pub fn cross_product() {
        let cross_product = Vector3::new(1.0, 2.0, 3.0).cross(&Vector3::new(3.0, 3.0, 3.0));
        assert_eq!(cross_product, Vector3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    pub fn scalar_product() {
        let scalar_product = Vector3::new(1.0, 2.0, -3.0) * 3.0;
        assert_eq!(scalar_product, Vector3::new(3.0, 6.0, -9.0));
    }

    #[test]
    pub fn mul_assign_scalar() {
        let mut vector = Vector3::new(1.0, 2.0, -3.0);
        vector *= 3.0;
        assert_eq!(vector, Vector3::new(3.0, 6.0, -9.0));
    }

    #[test]
    pub fn mul_assign_vector() {
        let mut vector = Vector3::new(1.0, 2.0, -3.0);
        vector *= Vector3::new(3.0, 3.0, 3.0);
        assert_eq!(vector, Vector3::new(3.0, 6.0, -9.0));
    }
}
