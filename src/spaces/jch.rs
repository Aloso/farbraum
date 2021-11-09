use crate::spaces::{util, Jab, Jch, Srgb};
use crate::{Color, Into};

impl Into<Jch> for Color<Jab> {
    fn into(self, _: Jch) -> Color<Jch> {
        let (j, a, b) = self.tuple();

        let c = (a * a + b * b).sqrt();
        let h = if c > 0.0 {
            util::normalize_hue(util::rad_to_deg(b.atan2(a)))
        } else {
            0.0
        };

        Color::of(j, c, h)
    }
}

impl Into<Jab> for Color<Jch> {
    fn into(self, _: Jab) -> Color<Jab> {
        let (j, c, h) = self.tuple();
        let (a, b) = util::cos_and_sin_radians(c, h);

        Color::of(j, a, b)
    }
}

impl Into<Jch> for Color<Srgb> {
    fn into(self, s: Jch) -> Color<Jch> {
        self.into(Jab).into(s)
    }
}

impl Into<Srgb> for Color<Jch> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(Jab).into(s)
    }
}
