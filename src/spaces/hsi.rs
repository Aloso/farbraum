use crate::spaces::{util, Hsi, Srgb};
use crate::{Color, From};

impl From<Hsi> for Color<Srgb> {
    fn from(hsi: Color<Hsi>) -> Self {
        let (h, s, i) = hsi.tuple();
        let h = util::normalize_hue(h);
        let f = (((h / 60.0) % 2.0) - 1.0).abs();

        match (h / 60.0).floor() as i32 {
            0 => Color::new(
                i * (1.0 + s * (3.0 / (2.0 - f) - 1.0)),
                i * (1.0 + s * ((3.0 * (1.0 - f)) / (2.0 - f) - 1.0)),
                i * (1.0 - s),
            ),
            1 => Color::new(
                i * (1.0 + s * ((3.0 * (1.0 - f)) / (2.0 - f) - 1.0)),
                i * (1.0 + s * (3.0 / (2.0 - f) - 1.0)),
                i * (1.0 - s),
            ),
            2 => Color::new(
                i * (1.0 - s),
                i * (1.0 + s * (3.0 / (2.0 - f) - 1.0)),
                i * (1.0 + s * ((3.0 * (1.0 - f)) / (2.0 - f) - 1.0)),
            ),
            3 => Color::new(
                i * (1.0 - s),
                i * (1.0 + s * ((3.0 * (1.0 - f)) / (2.0 - f) - 1.0)),
                i * (1.0 + s * (3.0 / (2.0 - f) - 1.0)),
            ),
            4 => Color::new(
                i * (1.0 + s * ((3.0 * (1.0 - f)) / (2.0 - f) - 1.0)),
                i * (1.0 - s),
                i * (1.0 + s * (3.0 / (2.0 - f) - 1.0)),
            ),
            5 => Color::new(
                i * (1.0 + s * (3.0 / (2.0 - f) - 1.0)),
                i * (1.0 - s),
                i * (1.0 + s * ((3.0 * (1.0 - f)) / (2.0 - f) - 1.0)),
            ),
            _ => Color::new(i * (1.0 - s), i * (1.0 - s), i * (1.0 - s)),
        }
    }
}

impl From<Srgb> for Color<Hsi> {
    fn from(rgb: Color<Srgb>) -> Self {
        let (r, g, b) = rgb.tuple();

        let (min, max) = util::min_max(r, g, b);
        let s = if r + g + b == 0.0 {
            0.0
        } else {
            1.0 - (3.0 * min) / (r + g + b)
        };
        let i = (r + g + b) / 3.0;
        let h = util::calculate_hsl_hue(r, g, b, max, min);

        Color::new(h, s, i)
    }
}
