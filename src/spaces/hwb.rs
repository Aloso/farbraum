use crate::spaces::{Hsv, Hwb, Srgb};
use crate::{Color, From};

impl From<Hwb> for Color<Hsv> {
    fn from(hwb: Color<Hwb>) -> Self {
        let (h, mut w, mut b) = hwb.tuple();

        // normalize w + b to 1
        if w + b > 1.0 {
            let s = w + b;
            w /= s;
            b /= s;
        }
        Color::new(h, if b == 1.0 { 1.0 } else { 1.0 - w / (1.0 - b) }, 1.0 - b)
    }
}

impl From<Hwb> for Color<Srgb> {
    fn from(hwb: Color<Hwb>) -> Self {
        hwb.into::<Hsv>().into()
    }
}

impl From<Hsv> for Color<Hwb> {
    fn from(hsv: Color<Hsv>) -> Self {
        let (h, s, v) = hsv.tuple();

        let w = (1.0 - s) * v;
        let b = 1.0 - v;

        Color::new(h, w, b)
    }
}

impl From<Srgb> for Color<Hwb> {
    fn from(rgb: Color<Srgb>) -> Self {
        rgb.into::<Hsv>().into()
    }
}
