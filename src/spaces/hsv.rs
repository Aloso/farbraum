use crate::spaces::{Hsv, Srgb};
use crate::{util, From, Color};

impl From<Hsv> for Color<Srgb> {
    fn from(hsv: Color<Hsv>) -> Self {
        let (h, s, v) = hsv.tuple();
        let h = util::normalize_hue(h);
        let f = (((h / 60.0) % 2.0) - 1.0).abs();

        match (h / 60.0).floor() as i32 {
            0 => Color::new(v, v * (1.0 - s * f), v * (1.0 - s)),
            1 => Color::new(v * (1.0 - s * f), v, v * (1.0 - s)),
            2 => Color::new(v * (1.0 - s), v, v * (1.0 - s * f)),
            3 => Color::new(v * (1.0 - s), v * (1.0 - s * f), v),
            4 => Color::new(v * (1.0 - s * f), v * (1.0 - s), v),
            5 => Color::new(v, v * (1.0 - s), v * (1.0 - s * f)),
            _ => Color::new(v * (1.0 - s), v * (1.0 - s), v * (1.0 - s)),
        }
    }
}

impl From<Srgb> for Color<Hsv> {
    fn from(rgb: Color<Srgb>) -> Self {
        let (r, g, b) = rgb.tuple();

        let (min, max) = util::min_max(r, g, b);
        let s = if max == 0.0 { 0.0 } else { 1.0 - min / max };
        let v = max;
        let h = util::calculate_hsl_hue(r, g, b, max, min);

        Color::new(h, s, v)
    }
}
