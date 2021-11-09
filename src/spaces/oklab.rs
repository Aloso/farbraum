use crate::spaces::{LinearSrgb, OkLab, Srgb};
use crate::{Color, Into};

impl Into<OkLab> for Color<LinearSrgb> {
    fn into(self, _: OkLab) -> Color<OkLab> {
        let (r, g, b) = self.tuple();

        let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
        let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
        let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();

        Color::of(
            0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s,
            1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s,
            0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s,
        )
    }
}

impl Into<OkLab> for Color<Srgb> {
    fn into(self, s: OkLab) -> Color<OkLab> {
        self.into(LinearSrgb).into(s)
    }
}

impl Into<LinearSrgb> for Color<OkLab> {
    fn into(self, _: LinearSrgb) -> Color<LinearSrgb> {
        let (l, a, b) = self.tuple();

        let l2 = (l + 0.3963377774 * a + 0.2158037573 * b).powi(3);
        let m = (l - 0.1055613458 * a - 0.0638541728 * b).powi(3);
        let s = (l - 0.0894841775 * a - 1.291485548 * b).powi(3);

        let r = 4.0767416621 * l2 - 3.3077115913 * m + 0.2309699292 * s;
        let g = -1.2684380046 * l2 + 2.6097574011 * m - 0.3413193965 * s;
        let b = -0.0041960863 * l2 - 0.7034186147 * m + 1.707614701 * s;

        Color::of(r, g, b)
    }
}

impl Into<Srgb> for Color<OkLab> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(LinearSrgb).into(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::spaces::OkLab;
    use crate::test_util::round_trips_srgb;

    #[test]
    fn test_oklab_roundtrips() {
        round_trips_srgb::<OkLab>();
    }
}
