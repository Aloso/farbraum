use crate::spaces::{CieLab, DLch};
use crate::whites::D65;
use crate::{util, Float, From, Vec3};

const PI: Float = float!(PI);
const E: Float = float!(E);
const KE: Float = 1.0;
const KCH: Float = 1.0;
const DELTA: Float = (26.0 / 180.0) * PI;

fn factor() -> Float {
    100.0 / (float!(139.0) / 100.0).log(E) // ~ 303.67
}

impl From<DLch> for Vec3<CieLab, D65> {
    fn from(lch: Vec3<DLch>) -> Self {
        let (l, c, h) = lch.tuple();

        let cos_delta = DELTA.cos();
        let sin_delta = DELTA.sin();

        let l2 = (E.powf((l * KE) / factor()) - 1.0) / 0.0039;

        let g = ((0.0435 * c * KCH * KE).log(E) - 1.0) / 0.075;
        let e = g * ((h / 180.0) * PI - DELTA).cos();
        let f = g * ((h / 180.0) * PI - DELTA).sin();

        let a = e * cos_delta - (f / 0.83) * sin_delta;
        let b = e * sin_delta + (f / 0.83) * cos_delta;

        Vec3::new(l2, a, b)
    }
}

impl From<CieLab, D65> for Vec3<DLch> {
    fn from(lab: Vec3<CieLab, D65>) -> Self {
        let (l, a, b) = lab.tuple();

        let cos_delta = DELTA.cos();
        let sin_delta = DELTA.sin();

        let e = a * cos_delta + b * sin_delta;
        let f = 0.83 * (b * cos_delta - a * sin_delta);
        let g = (e * e + f * f).sqrt();

        let l2 = (factor() / KE) * (1.0 + 0.0039 * l).log(E);
        let c = (1.0 + 0.075 * g).log(E) / (0.0435 * KCH * KE);
        let h = if c != 0.0 {
            util::normalize_hue(((f.atan2(e) + DELTA) / PI) * 180.0)
        } else {
            0.0
        };
        Vec3::new(l2, c, h)
    }
}
