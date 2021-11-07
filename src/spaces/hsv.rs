use crate::spaces::{Hsv, Srgb};
use crate::{util, From, Vec3};

impl From<Hsv> for Vec3<Srgb> {
    fn from(hsv: Vec3<Hsv>) -> Self {
        let (h, s, v) = hsv.tuple();
        let h = util::normalize_hue(h);
        let f = (((h / 60.0) % 2.0) - 1.0).abs();

        match (h / 60.0).floor() as i32 {
            0 => Vec3::new(v, v * (1.0 - s * f), v * (1.0 - s)),
            1 => Vec3::new(v * (1.0 - s * f), v, v * (1.0 - s)),
            2 => Vec3::new(v * (1.0 - s), v, v * (1.0 - s * f)),
            3 => Vec3::new(v * (1.0 - s), v * (1.0 - s * f), v),
            4 => Vec3::new(v * (1.0 - s * f), v * (1.0 - s), v),
            5 => Vec3::new(v, v * (1.0 - s), v * (1.0 - s * f)),
            _ => Vec3::new(v * (1.0 - s), v * (1.0 - s), v * (1.0 - s)),
        }
    }
}

impl From<Srgb> for Vec3<Hsv> {
    fn from(rgb: Vec3<Srgb>) -> Self {
        let (r, g, b) = rgb.tuple();

        let (min, max) = util::min_max(r, g, b);
        let s = if max == 0.0 { 0.0 } else { 1.0 - min / max };
        let v = max;
        let h = util::calculate_hsl_hue(r, g, b, max, min);

        Vec3::new(h, s, v)
    }
}
