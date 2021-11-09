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

#[cfg(test)]
mod tests {

    use crate::spaces::{Hsi, Srgb};
    use crate::test_util::round_trips_srgb;
    use crate::{Color, Float};

    fn rgb(r: Float, g: Float, b: Float) -> Color<Srgb> {
        Color::new(r, g, b)
    }

    fn hsi(l: Float, a: Float, b: Float) -> Color<Hsi> {
        Color::new(l, a, b)
    }

    #[test]
    fn test_hsi_to_rgb() {
        assert_eq!(hsi(0.0, 0.0, 0.0).into(), rgb(0.0, 0.0, 0.0));
        assert_eq!(hsi(60.0, 0.25, 0.0).into(), rgb(0.0, 0.0, 0.0));
        assert_eq!(hsi(0.0, 0.0, 0.5).into(), rgb(0.5, 0.5, 0.5));
        assert_eq!(hsi(60.0, 0.0, 0.25).into(), rgb(0.25, 0.25, 0.25));
        assert_eq!(hsi(100.0, 0.0, 0.5).into(), rgb(0.5, 0.5, 0.5));
    }

    #[test]
    fn test_rgb_to_hsi() {
        assert_eq!(hsi(0.0, 0.0, 0.0), rgb(0.0, 0.0, 0.0).into());
        assert_eq!(hsi(0.0, 0.0, 0.25), rgb(0.25, 0.25, 0.25).into());

        // red, yellow, green, cyan, blue, magenta
        assert_eq!(hsi(0.0, 1.0, 1.0 / 3.0), rgb(1.0, 0.0, 0.0).into());
        assert_eq!(hsi(60.0, 1.0, 2.0 / 3.0), rgb(1.0, 1.0, 0.0).into());
        assert_eq!(hsi(120.0, 1.0, 1.0 / 3.0), rgb(0.0, 1.0, 0.0).into());
        assert_eq!(hsi(180.0, 1.0, 2.0 / 3.0), rgb(0.0, 1.0, 1.0).into());
        assert_eq!(hsi(240.0, 1.0, 1.0 / 3.0), rgb(0.0, 0.0, 1.0).into());
        assert_eq!(hsi(300.0, 1.0, 2.0 / 3.0), rgb(1.0, 0.0, 1.0).into());
    }

    #[test]
    fn test_hsi_roundtrips() {
        round_trips_srgb::<Hsi>();
    }
}
