use core::{fmt, marker::PhantomData};

use crate::{Float, From};

/// A color in a specific color space, consisting of 3 [`Float`] components.
///
/// The color spaces are just phantom types. They ensure that only the correct
/// conversion functions can be applied to a color. Conversion is handled by
/// the [`into()`](#methods.into) method.
pub struct Color<SPACE>(Float, Float, Float, PhantomData<SPACE>);

impl<S> Color<S> {
    /// Create a new color. The color components must be provided in the correct order,
    /// e.g. `Color::new(r, g, b)` or `Color::new(x, y, z)`.
    #[allow(clippy::just_underscores_and_digits)]
    pub const fn new(_0: Float, _1: Float, _2: Float) -> Self {
        Color(_0, _1, _2, PhantomData)
    }

    /// Returns the components as a tuple.
    pub const fn tuple(self) -> (Float, Float, Float) {
        (self.0, self.1, self.2)
    }

    /// Converts the color to a different color space.
    ///
    /// ### Example
    ///
    /// ```
    /// use farbraum::Color;
    /// use farbraum::spaces::{Hsv, Srgb, LinearSrgb};
    ///
    /// let hsv: Color<Hsv> = Color::new(120.0, 1.0, 1.0);
    /// let l_rgb = hsv.into::<Srgb>().into::<LinearSrgb>();
    ///
    /// assert_eq!(l_rgb, Color::new(0.0, 1.0, 0.0));
    /// ```
    ///
    /// **Implementation note:**
    ///
    /// Conversion uses the [`From`] trait. This is _not_ the same trait
    /// as `From` from the standard library.
    pub fn into<SPACE>(self) -> Color<SPACE>
    where
        Color<SPACE>: From<S>,
    {
        From::from(self)
    }

    pub fn ref_0(&self) -> &Float {
        &self.0
    }

    pub fn ref_1(&self) -> &Float {
        &self.1
    }

    pub fn ref_2(&self) -> &Float {
        &self.2
    }

    pub fn mut_0(&mut self) -> &mut Float {
        &mut self.0
    }

    pub fn mut_1(&mut self) -> &mut Float {
        &mut self.1
    }

    pub fn mut_2(&mut self) -> &mut Float {
        &mut self.2
    }
}

impl<SPACE> Copy for Color<SPACE> {}

impl<SPACE> Clone for Color<SPACE> {
    fn clone(&self) -> Self {
        Self(self.0, self.1, self.2, PhantomData)
    }
}

impl<SPACE: fmt::Debug + Default> fmt::Debug for Color<SPACE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}({}, {}, {})",
            SPACE::default(),
            self.0,
            self.1,
            self.2
        )
    }
}

impl<SPACE> Default for Color<SPACE> {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0, PhantomData)
    }
}

impl<SPACE> PartialEq for Color<SPACE> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
