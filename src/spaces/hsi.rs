use crate::spaces::{Hsi, Srgb};
use crate::{util, From, Vec3};

impl From<Hsi> for Vec3<Srgb> {
    fn from(hsi: Vec3<Hsi>) -> Self {
        let (h, s, i) = hsi.tuple();
        let h = util::normalize_hue(h);
        let f = (((h / 60.0) % 2.0) - 1.0).abs();

        match (h / 60.0).floor() as i32 {
            0 => Vec3::new(
                i * (1.0 + s * (3.0 / (2.0 - f) - 1.0)),
                i * (1.0 + s * ((3.0 * (1.0 - f)) / (2.0 - f) - 1.0)),
                i * (1.0 - s),
            ),
            1 => Vec3::new(
                i * (1.0 + s * ((3.0 * (1.0 - f)) / (2.0 - f) - 1.0)),
                i * (1.0 + s * (3.0 / (2.0 - f) - 1.0)),
                i * (1.0 - s),
            ),
            2 => Vec3::new(
                i * (1.0 - s),
                i * (1.0 + s * (3.0 / (2.0 - f) - 1.0)),
                i * (1.0 + s * ((3.0 * (1.0 - f)) / (2.0 - f) - 1.0)),
            ),
            3 => Vec3::new(
                i * (1.0 - s),
                i * (1.0 + s * ((3.0 * (1.0 - f)) / (2.0 - f) - 1.0)),
                i * (1.0 + s * (3.0 / (2.0 - f) - 1.0)),
            ),
            4 => Vec3::new(
                i * (1.0 + s * ((3.0 * (1.0 - f)) / (2.0 - f) - 1.0)),
                i * (1.0 - s),
                i * (1.0 + s * (3.0 / (2.0 - f) - 1.0)),
            ),
            5 => Vec3::new(
                i * (1.0 + s * (3.0 / (2.0 - f) - 1.0)),
                i * (1.0 - s),
                i * (1.0 + s * ((3.0 * (1.0 - f)) / (2.0 - f) - 1.0)),
            ),
            _ => Vec3::new(i * (1.0 - s), i * (1.0 - s), i * (1.0 - s)),
        }
    }
}

impl From<Srgb> for Vec3<Hsi> {
    fn from(rgb: Vec3<Srgb>) -> Self {
        let (r, g, b) = rgb.tuple();

        let (min, max) = util::min_max(r, g, b);
        let s = if r + g + b == 0.0 {
            0.0
        } else {
            1.0 - (3.0 * min) / (r + g + b)
        };
        let i = (r + g + b) / 3.0;
        let h = util::calculate_hsl_hue(r, g, b, max, min);

        Vec3::new(h, s, i)
    }
}
