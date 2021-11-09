use crate::spaces::{util, Hsv, Srgb};
use crate::{Color, Into};

impl Into<Srgb> for Color<Hsv> {
    fn into(self, _: Srgb) -> Color<Srgb> {
        let (h, s, v) = self.tuple();
        let h = util::normalize_hue(h);
        let f = (((h / 60.0) % 2.0) - 1.0).abs();

        match (h / 60.0).floor() as i32 {
            0 => Color::of(v, v * (1.0 - s * f), v * (1.0 - s)),
            1 => Color::of(v * (1.0 - s * f), v, v * (1.0 - s)),
            2 => Color::of(v * (1.0 - s), v, v * (1.0 - s * f)),
            3 => Color::of(v * (1.0 - s), v * (1.0 - s * f), v),
            4 => Color::of(v * (1.0 - s * f), v * (1.0 - s), v),
            5 => Color::of(v, v * (1.0 - s), v * (1.0 - s * f)),
            _ => Color::of(v * (1.0 - s), v * (1.0 - s), v * (1.0 - s)),
        }
    }
}

impl Into<Hsv> for Color<Srgb> {
    fn into(self, _: Hsv) -> Color<Hsv> {
        let (r, g, b) = self.tuple();

        let (min, max) = util::min_max(r, g, b);
        let s = if max == 0.0 { 0.0 } else { 1.0 - min / max };
        let v = max;
        let h = util::calculate_hsl_hue(r, g, b, max, min);

        Color::of(h, s, v)
    }
}

#[cfg(test)]
mod tests {

    use crate::spaces::{Hsv, Srgb};
    use crate::test_util::round_trips_srgb;
    use crate::{Color, Float};

    fn rgb(r: Float, g: Float, b: Float) -> Color<Srgb> {
        Color::of(r, g, b)
    }

    fn hsv(l: Float, a: Float, b: Float) -> Color<Hsv> {
        Color::of(l, a, b)
    }

    #[test]
    fn test_hsv_to_rgb() {
        assert_eq!(hsv(0.0, 0.0, 0.0).into(Srgb), rgb(0.0, 0.0, 0.0));
        assert_eq!(hsv(60.0, 0.25, 0.0).into(Srgb), rgb(0.0, 0.0, 0.0));
        assert_eq!(hsv(0.0, 0.0, 0.5).into(Srgb), rgb(0.5, 0.5, 0.5));
        assert_eq!(hsv(60.0, 0.0, 0.25).into(Srgb), rgb(0.25, 0.25, 0.25));
        assert_eq!(hsv(100.0, 0.0, 0.5).into(Srgb), rgb(0.5, 0.5, 0.5));
    }

    #[test]
    fn test_rgb_to_hsv() {
        assert_eq!(hsv(0.0, 0.0, 0.0), rgb(0.0, 0.0, 0.0).into(Hsv));
        assert_eq!(hsv(0.0, 0.0, 0.25), rgb(0.25, 0.25, 0.25).into(Hsv));
        assert_eq!(hsv(0.0, 0.0, 0.6), rgb(0.6, 0.6, 0.6).into(Hsv));

        // red, yellow, green, cyan, blue, magenta
        assert_eq!(hsv(0.0, 1.0, 1.0), rgb(1.0, 0.0, 0.0).into(Hsv));
        assert_eq!(hsv(60.0, 1.0, 1.0), rgb(1.0, 1.0, 0.0).into(Hsv));
        assert_eq!(hsv(120.0, 1.0, 1.0), rgb(0.0, 1.0, 0.0).into(Hsv));
        assert_eq!(hsv(180.0, 1.0, 1.0), rgb(0.0, 1.0, 1.0).into(Hsv));
        assert_eq!(hsv(240.0, 1.0, 1.0), rgb(0.0, 0.0, 1.0).into(Hsv));
        assert_eq!(hsv(300.0, 1.0, 1.0), rgb(1.0, 0.0, 1.0).into(Hsv));
    }

    #[test]
    fn test_hsv_roundtrips() {
        round_trips_srgb::<Hsv>();
    }
}
