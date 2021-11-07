#[cfg(feature = "double_precision")]
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

#[cfg(not(feature = "double_precision"))]
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

#[cfg(feature = "serde")]
mod serde_impl;

mod lab_util;
mod util;
mod vec3;

pub type Float = float!();
pub use vec3::Color;

pub mod spaces;
pub mod whites;

pub trait From<SPACE> {
    fn from(xyz: Color<SPACE>) -> Self;
}
