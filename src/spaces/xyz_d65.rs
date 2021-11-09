use crate::illuminate::{D50, D65};
use crate::spaces::{CieXyz, LinearSrgb, Srgb};
use crate::{Color, Into};

impl Into<CieXyz<D65>> for Color<CieXyz<D50>> {
    fn into(self, _: CieXyz<D65>) -> Color<CieXyz<D65>> {
        let (x, y, z) = self.tuple();

        let x2 = 0.9555766 * x - 0.0230393 * y + 0.0631636 * z;
        let y2 = -0.0282895 * x + 1.0099416 * y + 0.0210077 * z;
        let z2 = 0.0122982 * x - 0.020483 * y + 1.3299098 * z;

        Color::of(x2, y2, z2)
    }
}

impl Into<CieXyz<D50>> for Color<CieXyz<D65>> {
    fn into(self, _: CieXyz<D50>) -> Color<CieXyz<D50>> {
        let (x, y, z) = self.tuple();

        let x2 = 1.0478112 * x + 0.0228866 * y - 0.050127 * z;
        let y2 = 0.0295424 * x + 0.9904844 * y - 0.0170491 * z;
        let z2 = -0.0092345 * x + 0.0150436 * y + 0.7521316 * z;

        Color::of(x2, y2, z2)
    }
}

impl Into<CieXyz<D65>> for Color<LinearSrgb> {
    fn into(self, _: CieXyz<D65>) -> Color<CieXyz<D65>> {
        let (r, g, b) = self.tuple();

        let x = 0.412456432268 * r + 0.35757607628 * g + 0.180437480294 * b;
        let y = 0.212672846318 * r + 0.715152167155 * g + 0.072174999573 * b;
        let z = 0.019333904103 * r + 0.119192028243 * g + 0.950304073677 * b;

        Color::of(x, y, z)
    }
}

impl Into<CieXyz<D65>> for Color<Srgb> {
    fn into(self, s: CieXyz<D65>) -> Color<CieXyz<D65>> {
        self.into(LinearSrgb).into(s)
    }
}

impl Into<LinearSrgb> for Color<CieXyz<D65>> {
    fn into(self, _: LinearSrgb) -> Color<LinearSrgb> {
        let (x, y, z) = self.tuple();

        let r = x * 3.2404542 - y * 1.5371385 - 0.4985314 * z;
        let g = x * -0.969266 + y * 1.8760108 + 0.041556 * z;
        let b = x * 0.0556434 - y * 0.2040259 + 1.0572252 * z;

        Color::of(r, g, b)
    }
}

impl Into<Srgb> for Color<CieXyz<D65>> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(LinearSrgb).into(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::illuminate::D65;
    use crate::spaces::CieXyz;
    use crate::test_util::round_trips_srgb;

    #[test]
    fn test_xyz_d65_roundtrips() {
        round_trips_srgb::<CieXyz<D65>>();
    }
}
