#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

pub mod particle;
pub mod vec;

pub type Real = f64;

pub use self::{particle::*, vec::*};

pub(crate) fn assert_equal(actual: Real, expected: Real) {
    assert!(
        (actual - expected).abs() < Real::EPSILON,
        "left: {:?} not equal right: {:?}",
        actual,
        expected
    );
}
