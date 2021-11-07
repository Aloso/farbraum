use crate::Float;

/// Whitepoint D50. Commonly used by CIE color spaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct D50;

/// Whitepoint D65. Used by sRGB and many other color spaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct D65;

pub(crate) const D50_WHITE: (Float, Float, Float) = (0.96422, 1.0, 0.82521);

pub(crate) const D65_WHITE: (Float, Float, Float) = (0.95047, 1.0, 1.08883);
