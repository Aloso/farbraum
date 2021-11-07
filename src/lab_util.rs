#![macro_use]

use crate::{spaces::Lab, util, Float, Color};

const PI: Float = float!(PI);

pub(crate) fn lab_to_lch<LAB: Lab>(lab: Color<LAB>) -> Color<LAB::Lch> {
    let (l, a, b) = lab.tuple();

    let c = (a * a + b * b).sqrt();
    let h = if c != 0.0 {
        util::normalize_hue((b.atan2(a) * 180.0) / PI)
    } else {
        0.0
    };

    Color::new(l, c, h)
}

pub(crate) fn lch_to_lab<LAB: Lab>(lch: Color<LAB::Lch>) -> Color<LAB> {
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

    Color::new(l, a, b)
}
