#![allow(clippy::float_cmp)]

//! # farbraum
//!
//! Rust crate to convert between color spaces. "Farbraum"
//! [/ˈfarbraʊ̯m/](http://ipa-reader.xyz/?text=%CB%88farbra%CA%8A%CC%AFm&voice=Marlene)
//! is German for "color space".
//!
//! Most conversion functions are ported from the [culori](https://culorijs.org)
//! javascript library. Some parts were modified to make the results more accurate.
//!
//! ## Usage
//!
//! Colors are created with `Color::new()`, the last argument is the color space.
//! The `.into()` method is used to convert between color spaces:
//!
//! ```
//! use farbraum::{Color, spaces::{Srgb, LinearSrgb, Hsv}};
//!
//! let hsv = Color::new(120.0, 1.0, 1.0, Hsv);
//! let lrgb = hsv.into(Srgb).into(LinearSrgb);
//! let (r, g, b) = lrgb.tuple();
//! ```
//!
//! ## Color spaces
//!
//! Farbraum supports 24 color spaces:
//!
//! * sRGB, Linear sRGB
//! * Adobe RGB (1998)
//! * CMY, CMYK<sup>1</sup>
//! * sRGB-derived color spaces (HSL, HSV, HSI, HWB)
//! * CIE XYZ (supports D50 and D65 illumination)
//! * CIELAB, CIELCh (supports D50 and D65 illumination)
//! * CIELUV, CIELCh<sub>uv</sub> (D50 illumination)
//! * Oklab, Oklch
//! * DIN99 Lab, DIN99 LCh
//! * J<sub>z</sub>a<sub>z</sub>b<sub>z</sub>, J<sub>z</sub>C<sub>z</sub>h<sub>z</sub>
//! * Cubehelix
//!
//! All color spaces use the
//! [D65 standard illuminate](https://en.wikipedia.org/wiki/Illuminant_D65)
//! unless stated otherwise. CIE XYZ, CIELAB and CIELCh are available
//! with both D50 and D65 illuminant.
//!
//! <sup>1</sup> Since CMYK has 4 components instead of 3, it must be
//! represented as a tuple instead of a `Color`. Use the `{to,from}_cmyk()`
//! methods to convert between `Color<CMY>` and `(C, M, Y, K)`.
//!
//! ## Dynamic color spaces
//!
//! The color spaces are zero-sized types, so they don't exist at runtime. If you
//! want to choose a color space at runtime, you'll need to create an enum such as:
//!
//! ```
//! #[derive(Debug, Clone, Copy)]
//! enum AnyColorSpace {
//!     CieXyzD50,
//!     Srgb,
//!     Hsl,
//!     CieLabD50,
//!     OkLab,
//! }
//! ```
//!
//! This can be used instead of the `farbraum`'s builtin color spaces. However,
//! you'll need to implement conversions for this enum to make it useful:
//!
//! ```
//! use farbraum::{
//!     Color, illuminate::D50,
//!     spaces::{Srgb, CieXyz, Hsl, CieLab, OkLab},
//! };
//! #
//! # #[derive(Debug, Clone, Copy)]
//! # enum AnyColorSpace {
//! #     CieXyzD50,
//! #     Srgb,
//! #     Hsl,
//! #     CieLabD50,
//! #     OkLab,
//! # }
//!
//! // Convert any color space to sRGB
//! fn any_to_srgb(any: Color<AnyColorSpace>) -> Color<Srgb> {
//!     let (a, b, c) = any.tuple();
//!     match any.space() {
//!         AnyColorSpace::Srgb      => Color::new(a, b, c, Srgb),
//!         AnyColorSpace::CieXyzD50 => Color::new(a, b, c, CieXyz(D50)).into(Srgb),
//!         AnyColorSpace::Hsl       => Color::new(a, b, c, Hsl).into(Srgb),
//!         AnyColorSpace::CieLabD50 => Color::new(a, b, c, CieLab(D50)).into(Srgb),
//!         AnyColorSpace::OkLab     => Color::new(a, b, c, OkLab).into(Srgb),
//!     }
//! }
//! ```
//!
//! ## Cargo features
//!
//! - `double-precision`: Components are floating-point values, by default `f64`. If
//!   you disable the `double-precision` feature, `f32` is used instead.
//! - `serde`: Enable this feature to serialize and deserialize `Color` values.
//!
//! ## License
//!
//! Dual-licensed under the **Apache 2.0** and **MIT** license.

#[cfg(feature = "double-precision")]
macro_rules! float {
    () => {
        f64
    };
    ($lit:literal) => {
        $lit as f64
    };
    ($constant:ident) => {
        std::f64::consts::$constant
    };
}

#[cfg(not(feature = "double-precision"))]
macro_rules! float {
    () => {
        f32
    };
    ($lit:literal) => {
        $lit as f32
    };
    ($constant:ident) => {
        std::f32::consts::$constant
    };
}

#[cfg(test)]
mod test_util;

mod color;

/// The color component type, `f64` by default.
///
/// If you disable the `double-precision` feature, `f32` is used instead.
pub type Float = float!();
pub use color::Color;

pub mod illuminate;
pub mod spaces;

/// Trait for color space conversions.
pub trait Into<SPACE> {
    fn into(self, space: SPACE) -> Color<SPACE>;
}
