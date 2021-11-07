#![macro_use]

use crate::{spaces::Lab, util, Float, Vec3};

const PI: Float = float!(PI);

pub(crate) fn lab_to_lch<LAB: Lab, WHITE>(lab: Vec3<LAB, WHITE>) -> Vec3<LAB::Lch, WHITE> {
    let (l, a, b) = lab.tuple();

    let c = (a * a + b * b).sqrt();
    let h = if c != 0.0 {
        util::normalize_hue((b.atan2(a) * 180.0) / PI)
    } else {
        0.0
    };

    Vec3::new(l, c, h)
}

pub(crate) fn lch_to_lab<LAB: Lab, WHITE>(lch: Vec3<LAB::Lch, WHITE>) -> Vec3<LAB, WHITE> {
    let (l, c, h) = lch.tuple();

    let a = if c != 0.0 {
        c * ((h / 180.0) * PI).cos()
    } else {
        0.0
    };
    let b = if c != 0.0 {
        c * ((h / 180.0) * PI).sin()
    } else {
        0.0
    };

    Vec3::new(l, a, b)
}
