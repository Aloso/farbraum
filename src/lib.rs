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

#[cfg(test)]
mod test_util;

mod color;

pub type Float = float!();
pub use color::Color;

pub mod illuminate;
pub mod spaces;

pub trait From<SPACE> {
    fn from(xyz: Color<SPACE>) -> Self;
}
