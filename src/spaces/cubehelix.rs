use crate::spaces::Srgb;
use crate::{Float, From, Vec3};

use super::CubeHelix;

pub(crate) const M: [Float; 6] = [-0.14861, 1.78277, -0.29227, -0.90649, 1.97294, 0.0];
const DE: Float = M[3] * M[4];
const BE: Float = M[1] * M[4];
const BCAD: Float = M[1] * M[2] - M[0] * M[3];

pub(crate) const DEG_TO_RAD: Float = float!(PI) / 180.0;
pub(crate) const RAD_TO_DEG: Float = 180.0 / float!(PI);

impl From<CubeHelix> for Vec3<Srgb> {
    fn from(cubehelix: Vec3<CubeHelix>) -> Self {
        let (h, s, l) = cubehelix.tuple();

        let h = (h + 120.0) * DEG_TO_RAD;
        let amp = s * l * (1.0 - l);

        let cosh = h.cos();
        let sinh = h.sin();

        Vec3::new(
            l + amp * (M[0] * cosh + M[1] * sinh),
            l + amp * (M[2] * cosh + M[3] * sinh),
            l + amp * (M[4] * cosh + M[5] * sinh),
        )
    }
}

impl From<Srgb> for Vec3<CubeHelix> {
    fn from(rgb: Vec3<Srgb>) -> Self {
        let (red, green, blue) = rgb.tuple();
        let l = (BCAD * blue + red * DE - green * BE) / (BCAD + DE - BE);

        let x = blue - l;
        let y = (M[4] * (green - l) - M[2] * x) / M[3];

        let s = if l == 0.0 || l == 1.0 {
            0.0
        } else {
            (x * x + y * y).sqrt() / (M[4] * l * (1.0 - l))
        };

        let h = if s != 0.0 {
            y.atan2(x) * RAD_TO_DEG - 120.0
        } else {
            0.0
        };

        Vec3::new(h, s, l)
    }
}
