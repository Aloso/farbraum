use crate::spaces::{LinearSrgb, Srgb};
use crate::{Float, From, Color};

fn positive_signum(n: Float) -> Float {
    if n >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

fn f1(c: Float) -> Float {
    let abs = c.abs();
    if abs > 0.0031308 {
        positive_signum(c) * (1.055 * abs.powf(1.0 / 2.4) - 0.055)
    } else {
        c * 12.92
    }
}

impl From<LinearSrgb> for Color<Srgb> {
    fn from(rgb: Color<LinearSrgb>) -> Self {
        let (r, g, b) = rgb.tuple();
        Color::new(f1(r), f1(g), f1(b))
    }
}

fn f2(c: Float) -> Float {
    let abs = c.abs();
    if abs < 0.04045 {
        c / 12.92
    } else {
        positive_signum(c) * ((abs + 0.055) / 1.055).powf(2.4)
    }
}

impl From<Srgb> for Color<LinearSrgb> {
    fn from(rgb: Color<Srgb>) -> Self {
        let (r, g, b) = rgb.tuple();
        Color::new(f2(r), f2(g), f2(b))
    }
}
