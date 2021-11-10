//! Contains the D50 and D65 standard illuminants.

use crate::Float;

/// Standard D50 illuminate. Commonly used by CIE color spaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct D50;

/// Standard D65 illuminate. Used by most color spaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct D65;

pub(crate) const D50_WHITE: (Float, Float, Float) = (0.96422, 1.0, 0.82521);

pub(crate) const D65_WHITE: (Float, Float, Float) = (0.95047, 1.0, 1.08883);
