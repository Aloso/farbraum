use crate::spaces::{Cmy, Srgb};
use crate::{Color, Into};

impl Into<Srgb> for Color<Cmy> {
    fn into(self, _: Srgb) -> Color<Srgb> {
        let (c, m, y) = self.tuple();
        Color::of(1.0 - c, 1.0 - m, 1.0 - y)
    }
}

impl Into<Cmy> for Color<Srgb> {
    fn into(self, _: Cmy) -> Color<Cmy> {
        let (r, g, b) = self.tuple();
        Color::of(1.0 - r, 1.0 - g, 1.0 - b)
    }
}
