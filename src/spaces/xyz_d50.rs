use crate::spaces::{CieXyz, LinearSrgb, Srgb};
use crate::whites::D50;
use crate::{Float, From, Vec3};

pub(crate) const K: Float = (29.0 * 29.0 * 29.0) / (3.0 * 3.0 * 3.0);
pub(crate) const E: Float = (6.0 * 6.0 * 6.0) / (29.0 * 29.0 * 29.0);

impl From<LinearSrgb> for Vec3<CieXyz, D50> {
    fn from(rgb: Vec3<LinearSrgb>) -> Self {
        let (r, g, b) = rgb.tuple();

        let x = 0.4360747 * r + 0.3850649 * g + 0.1430804 * b;
        let y = 0.2225045 * r + 0.7168786 * g + 0.0606169 * b;
        let z = 0.0139322 * r + 0.0971045 * g + 0.7141733 * b;

        Vec3::new(x, y, z)
    }
}

impl From<Srgb> for Vec3<CieXyz, D50> {
    fn from(rgb: Vec3<Srgb>) -> Self {
        rgb.into::<LinearSrgb, _>().into()
    }
}

impl From<CieXyz, D50> for Vec3<LinearSrgb> {
    fn from(xyz: Vec3<CieXyz, D50>) -> Self {
        let (x, y, z) = xyz.tuple();

        let r = x * 3.1338561 - y * 1.6168667 - 0.4906146 * z;
        let g = x * -0.9787684 + y * 1.9161415 + 0.033454 * z;
        let b = x * 0.0719453 - y * 0.2289914 + 1.4052427 * z;

        Vec3::new(r, g, b)
    }
}

impl From<CieXyz, D50> for Vec3<Srgb> {
    fn from(xyz: Vec3<CieXyz, D50>) -> Self {
        xyz.into::<LinearSrgb, _>().into()
    }
}
