use crate::illuminate::D65;
use crate::spaces::{lab, util, CieLab, DLab, DLch, Srgb};
use crate::{Color, Float, Into};

const PI: Float = float!(PI);
const E: Float = float!(E);
const KE: Float = 1.0;
const KCH: Float = 1.0;
const DELTA: Float = (26.0 / 180.0) * PI;

impl Into<DLch> for Color<DLab> {
    fn into(self, _: DLch) -> Color<DLch> {
        lab::lab_to_lch(self)
    }
}

impl Into<DLab> for Color<DLch> {
    fn into(self, _: DLab) -> Color<DLab> {
        lab::lch_to_lab(self)
    }
}

fn factor() -> Float {
    100.0 / (float!(139.0) / 100.0).log(E) // ~ 303.67
}

impl Into<CieLab<D65>> for Color<DLch> {
    fn into(self, _: CieLab<D65>) -> Color<CieLab<D65>> {
        let (l, c, h) = self.tuple();

        let cos_delta = DELTA.cos();
        let sin_delta = DELTA.sin();

        let l2 = (E.powf((l * KE) / factor()) - 1.0) / 0.0039;

        let g = (E.powf(0.0435 * c * KCH * KE) - 1.0) / 0.075;
        let e = g * (util::deg_to_rad(h) - DELTA).cos();
        let f = g * (util::deg_to_rad(h) - DELTA).sin();

        let a = e * cos_delta - (f / 0.83) * sin_delta;
        let b = e * sin_delta + (f / 0.83) * cos_delta;

        Color::of(l2, a, b)
    }
}

impl Into<DLch> for Color<CieLab<D65>> {
    fn into(self, _: DLch) -> Color<DLch> {
        let (l, a, b) = self.tuple();

        let cos_delta = DELTA.cos();
        let sin_delta = DELTA.sin();

        let e = a * cos_delta + b * sin_delta;
        let f = 0.83 * (b * cos_delta - a * sin_delta);
        let g = (e * e + f * f).sqrt();

        let l2 = (factor() / KE) * (1.0 + 0.0039 * l).log(E);
        let c = (1.0 + 0.075 * g).log(E) / (0.0435 * KCH * KE);
        let h = if c != 0.0 && !c.is_nan() {
            util::normalize_hue(((f.atan2(e) + DELTA) / PI) * 180.0)
        } else {
            0.0
        };
        Color::of(l2, c, h)
    }
}

impl Into<DLch> for Color<Srgb> {
    fn into(self, s: DLch) -> Color<DLch> {
        self.into(CieLab(D65)).into(s)
    }
}

impl Into<Srgb> for Color<DLch> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(CieLab(D65)).into(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::spaces::{DLab, DLch, Srgb};
    use crate::test_util::round_trips;
    use crate::{Color, Float};

    fn rgb(r: Float, g: Float, b: Float) -> Color<Srgb> {
        Color::of(r, g, b)
    }

    fn dlch(l: Float, a: Float, b: Float) -> Color<DLch> {
        Color::of(l, a, b)
    }

    #[test]
    fn test_dlch() {
        assert_eq!(
            rgb(1.0, 1.0, 1.0).into(DLch),
            dlch(100.00000042980086, 0.0, 0.0)
        );
        let x11 = 1.0 / 15.0;
        assert_eq!(
            rgb(x11, x11, x11).into(DLch),
            dlch(5.938147698096487, 0.0, 0.0)
        );
        assert_eq!(rgb(0.0, 0.0, 0.0).into(DLch), dlch(0.0, 0.0, 0.0));
        assert_eq!(
            rgb(1.0, 0.0, 0.0).into(DLch),
            dlch(57.29278122742003, 49.91494982539091, 37.691027859887654)
        );
    }

    #[test]
    fn test_dlch_roundtrips() {
        round_trips::<DLab, DLch>();
        round_trips::<DLch, DLab>();
    }
}
