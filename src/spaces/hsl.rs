use crate::spaces::{Hsl, Srgb};
use crate::{util, From, Vec3};

impl From<Hsl> for Vec3<Srgb> {
    fn from(hsl: Vec3<Hsl>) -> Self {
        let (h, s, l) = hsl.tuple();

        let h = util::normalize_hue(h);
        let m1 = l + s * (if l < 0.5 { l } else { 1.0 - l });
        let m2 = m1 - (m1 - l) * 2.0 * (((h / 60.0) % 2.0) - 1.0).abs();

        match (h / 60.0).floor() as i32 {
            0 => Vec3::new(m1, m2, 2.0 * l - m1),
            1 => Vec3::new(m2, m1, 2.0 * l - m1),
            2 => Vec3::new(2.0 * l - m1, m1, m2),
            3 => Vec3::new(2.0 * l - m1, m2, m1),
            4 => Vec3::new(m2, 2.0 * l - m1, m1),
            5 => Vec3::new(m1, 2.0 * l - m1, m2),
            _ => Vec3::new(2.0 * l - m1, 2.0 * l - m1, 2.0 * l - m1),
        }
    }
}

impl From<Srgb> for Vec3<Hsl> {
    fn from(rgb: Vec3<Srgb>) -> Self {
        let (r, g, b) = rgb.tuple();

        let (min, max) = util::min_max(r, g, b);
        let s = if max == min {
            0.0
        } else {
            (max - min) / (1.0 - (max + min - 1.0).abs())
        };
        let l = 0.5 * (max + min);
        let h = util::calculate_hsl_hue(r, g, b, max, min);

        Vec3::new(h, s, l)
    }
}
