use crate::spaces::{util, Hsl, Srgb};
use crate::{Color, From};

impl From<Hsl> for Color<Srgb> {
    fn from(hsl: Color<Hsl>) -> Self {
        let (h, s, l) = hsl.tuple();

        let h = util::normalize_hue(h);
        let m1 = l + s * (if l < 0.5 { l } else { 1.0 - l });
        let m2 = m1 - (m1 - l) * 2.0 * (((h / 60.0) % 2.0) - 1.0).abs();

        match (h / 60.0).floor() as i32 {
            0 => Color::new(m1, m2, 2.0 * l - m1),
            1 => Color::new(m2, m1, 2.0 * l - m1),
            2 => Color::new(2.0 * l - m1, m1, m2),
            3 => Color::new(2.0 * l - m1, m2, m1),
            4 => Color::new(m2, 2.0 * l - m1, m1),
            5 => Color::new(m1, 2.0 * l - m1, m2),
            _ => Color::new(2.0 * l - m1, 2.0 * l - m1, 2.0 * l - m1),
        }
    }
}

impl From<Srgb> for Color<Hsl> {
    fn from(rgb: Color<Srgb>) -> Self {
        let (r, g, b) = rgb.tuple();

        let (min, max) = util::min_max(r, g, b);
        let s = if max == min {
            0.0
        } else {
            (max - min) / (1.0 - (max + min - 1.0).abs())
        };
        let l = 0.5 * (max + min);
        let h = util::calculate_hsl_hue(r, g, b, max, min);

        Color::new(h, s, l)
    }
}

#[cfg(test)]
mod tests {

    use crate::spaces::{Hsl, Srgb};
    use crate::test_util::round_trips_srgb;
    use crate::{Color, Float};

    fn rgb(r: Float, g: Float, b: Float) -> Color<Srgb> {
        Color::new(r, g, b)
    }

    fn hsl(l: Float, a: Float, b: Float) -> Color<Hsl> {
        Color::new(l, a, b)
    }

    #[test]
    fn test_hsl_to_rgb() {
        assert_eq!(hsl(0.0, 0.0, 0.0).into(), rgb(0.0, 0.0, 0.0));
        assert_eq!(hsl(60.0, 0.25, 0.0).into(), rgb(0.0, 0.0, 0.0));
        assert_eq!(hsl(0.0, 0.0, 0.5).into(), rgb(0.5, 0.5, 0.5));
        assert_eq!(hsl(60.0, 0.0, 0.25).into(), rgb(0.25, 0.25, 0.25));
        assert_eq!(hsl(100.0, 0.0, 0.5).into(), rgb(0.5, 0.5, 0.5));
    }

    #[test]
    fn test_rgb_to_hsl() {
        assert_eq!(hsl(0.0, 0.0, 0.0), rgb(0.0, 0.0, 0.0).into());
        assert_eq!(hsl(0.0, 0.0, 0.25), rgb(0.25, 0.25, 0.25).into());

        // red, yellow, green, cyan, blue, magenta
        assert_eq!(hsl(0.0, 1.0, 0.5), rgb(1.0, 0.0, 0.0).into());
        assert_eq!(hsl(60.0, 1.0, 0.5), rgb(1.0, 1.0, 0.0).into());
        assert_eq!(hsl(120.0, 1.0, 0.5), rgb(0.0, 1.0, 0.0).into());
        assert_eq!(hsl(180.0, 1.0, 0.5), rgb(0.0, 1.0, 1.0).into());
        assert_eq!(hsl(240.0, 1.0, 0.5), rgb(0.0, 0.0, 1.0).into());
        assert_eq!(hsl(300.0, 1.0, 0.5), rgb(1.0, 0.0, 1.0).into());
    }

    #[test]
    fn test_hsl_roundtrips() {
        round_trips_srgb::<Hsl>();
    }
}
