use crate::spaces::{Hsv, Hwb, Srgb};
use crate::{Color, Into};

impl Into<Hsv> for Color<Hwb> {
    fn into(self, _: Hsv) -> Color<Hsv> {
        let (h, mut w, mut b) = self.tuple();

        // normalize w + b to 1
        if w + b > 1.0 {
            let s = w + b;
            w /= s;
            b /= s;
        }
        Color::of(h, if b == 1.0 { 1.0 } else { 1.0 - w / (1.0 - b) }, 1.0 - b)
    }
}

impl Into<Srgb> for Color<Hwb> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(Hsv).into(s)
    }
}

impl Into<Hwb> for Color<Hsv> {
    fn into(self, _: Hwb) -> Color<Hwb> {
        let (h, s, v) = self.tuple();

        let w = (1.0 - s) * v;
        let b = 1.0 - v;

        Color::of(h, w, b)
    }
}

impl Into<Hwb> for Color<Srgb> {
    fn into(self, s: Hwb) -> Color<Hwb> {
        self.into(Hsv).into(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::spaces::{Hsv, Hwb};
    use crate::test_util::round_trips;

    #[test]
    fn test_hwb_roundtrips() {
        round_trips::<Hwb, Hsv>();
    }
}
