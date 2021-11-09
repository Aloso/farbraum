use crate::illuminate::D50;
use crate::spaces::{CieXyz, LinearSrgb, Srgb};
use crate::{Color, Float, Into};

pub(crate) const K: Float = (29.0 * 29.0 * 29.0) / (3.0 * 3.0 * 3.0);
pub(crate) const E: Float = (6.0 * 6.0 * 6.0) / (29.0 * 29.0 * 29.0);

impl Into<CieXyz<D50>> for Color<LinearSrgb> {
    fn into(self, _: CieXyz<D50>) -> Color<CieXyz<D50>> {
        let (r, g, b) = self.tuple();

        let x = 0.436074716431 * r + 0.385064915333 * g + 0.143080380986 * b;
        let y = 0.222504478679 * r + 0.716878600214 * g + 0.060616923407 * b;
        let z = 0.013932173982 * r + 0.097104523966 * g + 0.714173283533 * b;

        Color::of(x, y, z)
    }
}

impl Into<CieXyz<D50>> for Color<Srgb> {
    fn into(self, s: CieXyz<D50>) -> Color<CieXyz<D50>> {
        self.into(LinearSrgb).into(s)
    }
}

impl Into<LinearSrgb> for Color<CieXyz<D50>> {
    fn into(self, _: LinearSrgb) -> Color<LinearSrgb> {
        let (x, y, z) = self.tuple();

        let r = x * 3.1338561 - y * 1.6168667 - 0.4906146 * z;
        let g = x * -0.9787684 + y * 1.9161415 + 0.033454 * z;
        let b = x * 0.0719453 - y * 0.2289914 + 1.4052427 * z;

        Color::of(r, g, b)
    }
}

impl Into<Srgb> for Color<CieXyz<D50>> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(LinearSrgb).into(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::illuminate::D50;
    use crate::spaces::CieXyz;
    use crate::test_util::round_trips_srgb;

    #[test]
    fn test_xyz_d50_roundtrips() {
        round_trips_srgb::<CieXyz<D50>>();
    }
}
