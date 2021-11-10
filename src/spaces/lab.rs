use crate::spaces::{util, Lab};
use crate::Color;

pub(crate) fn lab_to_lch<LAB: Lab>(lab: Color<LAB>, space: LAB::Lch) -> Color<LAB::Lch> {
    let (l, a, b) = lab.tuple();

    let c = (a * a + b * b).sqrt();
    let h = if c != 0.0 && !c.is_nan() {
        util::normalize_hue(util::rad_to_deg(b.atan2(a)))
    } else {
        0.0
    };

    Color::new(l, c, h, space)
}

pub(crate) fn lch_to_lab<LAB: Lab>(lch: Color<LAB::Lch>, space: LAB) -> Color<LAB> {
    let (l, c, h) = lch.tuple();
    let (a, b) = util::cos_and_sin_radians(c, h);

    Color::new(l, a, b, space)
}
