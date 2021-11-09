use crate::spaces::{LinearSrgb, Srgb};
use crate::{Color, Float, Into};

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

impl Into<Srgb> for Color<LinearSrgb> {
    fn into(self, _: Srgb) -> Color<Srgb> {
        let (r, g, b) = self.tuple();
        Color::of(f1(r), f1(g), f1(b))
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

impl Into<LinearSrgb> for Color<Srgb> {
    fn into(self, _: LinearSrgb) -> Color<LinearSrgb> {
        let (r, g, b) = self.tuple();
        Color::of(f2(r), f2(g), f2(b))
    }
}
