use core::fmt;

use crate::{Float, Into};

/// A color in a specific color space, consisting of 3 [`Float`] components.
///
/// The color spaces are just phantom types. They ensure that only the correct
/// conversion functions can be applied to a color. Conversion is handled by
/// the [`into()`](#methods.into) method.
#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color<SPACE>(Float, Float, Float, SPACE);

impl<S> Color<S> {
    /// Create a new color.
    ///
    /// The color components must be provided in the correct order, e.g.
    /// `Color::new(r, g, b)` or `Color::new(x, y, z)`.
    #[allow(clippy::just_underscores_and_digits)]
    pub fn of(_0: Float, _1: Float, _2: Float) -> Self
    where
        S: Default,
    {
        Color(_0, _1, _2, S::default())
    }

    /// Create a new color with a color space argument.
    ///
    /// The color components must be provided in the correct order, e.g.
    /// `Color::new(r, g, b, Srgb)` or `Color::new(x, y, z, Srgb)`.
    #[allow(clippy::just_underscores_and_digits)]
    pub const fn new(_0: Float, _1: Float, _2: Float, space: S) -> Self {
        Color(_0, _1, _2, space)
    }

    /// Returns the components as a tuple.
    pub const fn tuple(&self) -> (Float, Float, Float) {
        (self.0, self.1, self.2)
    }

    /// Returns the color space.
    pub const fn space(&self) -> &S {
        &self.3
    }

    /// Converts the color to a different color space.
    ///
    /// ### Example
    ///
    /// ```
    /// use farbraum::Color;
    /// use farbraum::spaces::{Hsv, Srgb, LinearSrgb};
    ///
    /// let hsv: Color<Hsv> = Color::of(120.0, 1.0, 1.0);
    /// let l_rgb = hsv.into(Srgb).into(LinearSrgb);
    ///
    /// assert_eq!(l_rgb, Color::of(0.0, 1.0, 0.0));
    /// ```
    ///
    /// **Implementation note:**
    ///
    /// Conversion uses the [`Into`] trait. This is _not_ the same trait
    /// as `Into` from the standard library.
    pub fn into<SPACE>(self, s: SPACE) -> Color<SPACE>
    where
        Color<S>: Into<SPACE>,
    {
        Into::into(self, s)
    }

    /// Borrow first component
    pub fn ref_0(&self) -> &Float {
        &self.0
    }

    /// Borrow second component
    pub fn ref_1(&self) -> &Float {
        &self.1
    }

    /// Borrow third component
    pub fn ref_2(&self) -> &Float {
        &self.2
    }

    /// Mutably borrow first component
    pub fn mut_0(&mut self) -> &mut Float {
        &mut self.0
    }

    /// Mutably borrow second component
    pub fn mut_1(&mut self) -> &mut Float {
        &mut self.1
    }

    /// Mutably borrow third component
    pub fn mut_2(&mut self) -> &mut Float {
        &mut self.2
    }
}

impl<SPACE: fmt::Debug> fmt::Debug for Color<SPACE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({}, {}, {})", self.3, self.0, self.1, self.2)
    }
}

impl<SPACE> PartialEq for Color<SPACE> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
