use crate::spaces::{Cmy, Srgb};
use crate::{Color, Float, Into};

impl Into<Srgb> for Color<Cmy> {
    fn into(self, _: Srgb) -> Color<Srgb> {
        let (c, m, y) = self.tuple();
        Color::of(1.0 - c, 1.0 - m, 1.0 - y)
    }
}

impl Into<Cmy> for Color<Srgb> {
    fn into(self, _: Cmy) -> Color<Cmy> {
        let (r, g, b) = self.tuple();
        Color::of(1.0 - r, 1.0 - g, 1.0 - b)
    }
}

impl Color<Cmy> {
    /// Convert a CMY color to CMYK.
    ///
    /// Unfortunately CMYK can't be represented as a `Color` struct because
    /// it has 4 components, so this method returns a 4-tuple.
    pub fn to_cmyk(self) -> (Float, Float, Float, Float) {
        let (c, m, y) = self.tuple();
        let key = c.min(m).min(y);
        let divisor = 1.0 - key;

        if divisor == 0.0 {
            // black
            return (0.0, 0.0, 0.0, 1.0);
        }

        let c = (c - key) / divisor;
        let m = (m - key) / divisor;
        let y = (y - key) / divisor;

        (c, m, y, key)
    }

    /// Convert a CMYK color to CMY.
    ///
    /// Unfortunately CMYK can't be represented as a `Color` struct because
    /// it has 4 components, so this method accepts a 4-tuple.
    pub fn from_cmyk((c, m, y, key): (Float, Float, Float, Float)) -> Self {
        let factor = 1.0 - key;

        let c = (c * factor) + key;
        let m = (m * factor) + key;
        let y = (y * factor) + key;

        Color::of(c, m, y)
    }
}

#[cfg(test)]
mod tests {
    use crate::spaces::{Cmy, Srgb};
    use crate::{Color, Float};

    macro_rules! assert_almost_eq4 {
        ($v1:expr, $v2:expr) => {{
            let _v1 = $v1;
            let _v2 = $v2;
            let (a, b, c, d) = _v1;
            let (e, f, g, h) = _v2;
            let _equal0 = (a - e).abs() < 0.00000000001;
            let _equal1 = (b - f).abs() < 0.00000000001;
            let _equal2 = (c - g).abs() < 0.00000000001;
            let _equal3 = (d - h).abs() < 0.00000000001;
            if !_equal0 || !_equal1 || !_equal2 || !_equal3 {
                eprintln!(
                    "  Values are sufficiently similar at position 1:{}, 2:{}, 3:{}, 4:{}\n",
                    _equal0, _equal1, _equal2, _equal3
                );
                assert_eq!(_v1, _v2);
            }
        }};
    }

    #[track_caller]
    fn test_rgb_cmyk((r, g, b): (Float, Float, Float), cmyk: (Float, Float, Float, Float)) {
        let rgb = Color::new(r, g, b, Srgb);
        assert_almost_eq4!(rgb.into(Cmy).to_cmyk(), cmyk);
        assert_almost_eq!(rgb, Color::from_cmyk(cmyk).into(Srgb));
    }

    #[test]
    fn test_rgb_to_cmyk() {
        test_rgb_cmyk((0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0));
        test_rgb_cmyk((0.01, 0.01, 0.01), (0.0, 0.0, 0.0, 0.99));
        test_rgb_cmyk((1.0, 0.3, 0.0), (0.0, 0.7, 1.0, 0.0));
        test_rgb_cmyk((1.0, 0.3, 0.1), (0.0, 0.7, 0.9, 0.0));
        test_rgb_cmyk((0.75, 0.3, 0.0), (0.0, 0.6, 1.0, 0.25));
        test_rgb_cmyk((0.5, 0.375, 0.25), (0.0, 0.25, 0.5, 0.5));
    }
}
