use crate::spaces::{util, Jab, Jch, Srgb};
use crate::{Color, From};

impl From<Jab> for Color<Jch> {
    fn from(jab: Color<Jab>) -> Self {
        let (j, a, b) = jab.tuple();

        let c = (a * a + b * b).sqrt();
        let h = if c > 0.0 {
            util::normalize_hue(util::rad_to_deg(b.atan2(a)))
        } else {
            0.0
        };

        Color::new(j, c, h)
    }
}

impl From<Jch> for Color<Jab> {
    fn from(jch: Color<Jch>) -> Self {
        let (j, c, h) = jch.tuple();
        let (a, b) = util::cos_and_sin_radians(c, h);

        Color::new(j, a, b)
    }
}

impl From<Srgb> for Color<Jch> {
    fn from(rgb: Color<Srgb>) -> Self {
        rgb.into::<Jab>().into()
    }
}

impl From<Jch> for Color<Srgb> {
    fn from(jch: Color<Jch>) -> Self {
        jch.into::<Jab>().into()
    }
}
