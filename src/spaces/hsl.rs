use crate::spaces::{util, Hsl, Srgb};
use crate::{Color, Into};

impl Into<Srgb> for Color<Hsl> {
    fn into(self, _: Srgb) -> Color<Srgb> {
        let (h, s, l) = self.tuple();

        let h = util::normalize_hue(h);
        let m1 = l + s * (if l < 0.5 { l } else { 1.0 - l });
        let m2 = m1 - (m1 - l) * 2.0 * (((h / 60.0) % 2.0) - 1.0).abs();

        match (h / 60.0).floor() as i32 {
            0 => Color::of(m1, m2, 2.0 * l - m1),
            1 => Color::of(m2, m1, 2.0 * l - m1),
            2 => Color::of(2.0 * l - m1, m1, m2),
            3 => Color::of(2.0 * l - m1, m2, m1),
            4 => Color::of(m2, 2.0 * l - m1, m1),
            5 => Color::of(m1, 2.0 * l - m1, m2),
            _ => Color::of(2.0 * l - m1, 2.0 * l - m1, 2.0 * l - m1),
        }
    }
}

impl Into<Hsl> for Color<Srgb> {
    fn into(self, _: Hsl) -> Color<Hsl> {
        let (r, g, b) = self.tuple();

        let (min, max) = util::min_max(r, g, b);
        let s = if max == min {
            0.0
        } else {
            (max - min) / (1.0 - (max + min - 1.0).abs())
        };
        let l = 0.5 * (max + min);
        let h = util::calculate_hsl_hue(r, g, b, max, min);

        Color::of(h, s, l)
    }
}

#[cfg(test)]
mod tests {

    use crate::spaces::{Hsl, Srgb};
    use crate::test_util::round_trips_srgb;
    use crate::{Color, Float};

    fn rgb(r: Float, g: Float, b: Float) -> Color<Srgb> {
        Color::of(r, g, b)
    }

    fn hsl(l: Float, a: Float, b: Float) -> Color<Hsl> {
        Color::of(l, a, b)
    }

    #[test]
    fn test_hsl_to_rgb() {
        assert_eq!(hsl(0.0, 0.0, 0.0).into(Srgb), rgb(0.0, 0.0, 0.0));
        assert_eq!(hsl(60.0, 0.25, 0.0).into(Srgb), rgb(0.0, 0.0, 0.0));
        assert_eq!(hsl(0.0, 0.0, 0.5).into(Srgb), rgb(0.5, 0.5, 0.5));
        assert_eq!(hsl(60.0, 0.0, 0.25).into(Srgb), rgb(0.25, 0.25, 0.25));
        assert_eq!(hsl(100.0, 0.0, 0.5).into(Srgb), rgb(0.5, 0.5, 0.5));
    }

    #[test]
    fn test_rgb_to_hsl() {
        assert_eq!(hsl(0.0, 0.0, 0.0), rgb(0.0, 0.0, 0.0).into(Hsl));
        assert_eq!(hsl(0.0, 0.0, 0.25), rgb(0.25, 0.25, 0.25).into(Hsl));

        // red, yellow, green, cyan, blue, magenta
        assert_eq!(hsl(0.0, 1.0, 0.5), rgb(1.0, 0.0, 0.0).into(Hsl));
        assert_eq!(hsl(60.0, 1.0, 0.5), rgb(1.0, 1.0, 0.0).into(Hsl));
        assert_eq!(hsl(120.0, 1.0, 0.5), rgb(0.0, 1.0, 0.0).into(Hsl));
        assert_eq!(hsl(180.0, 1.0, 0.5), rgb(0.0, 1.0, 1.0).into(Hsl));
        assert_eq!(hsl(240.0, 1.0, 0.5), rgb(0.0, 0.0, 1.0).into(Hsl));
        assert_eq!(hsl(300.0, 1.0, 0.5), rgb(1.0, 0.0, 1.0).into(Hsl));
    }

    #[test]
    fn test_hsl_roundtrips() {
        round_trips_srgb::<Hsl>();
    }
}
