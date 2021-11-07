use crate::spaces::{CieLab, CieXyz, Srgb};
use crate::whites::{D50, D50_WHITE as WHITE};
use crate::{From, Color};

impl From<CieLab<D50>> for Color<CieXyz<D50>> {
    fn from(lab: Color<CieLab<D50>>) -> Self {
        let (l, a, b) = lab.tuple();

        fn f(v: crate::Float) -> crate::Float {
            let pow3 = v * v * v;
            if pow3 > crate::spaces::xyz_d50::E {
                pow3
            } else {
                (116.0 * v - 16.0) / crate::spaces::xyz_d50::K
            }
        }

        let fy = (l + 16.0) / 116.0;
        let fx = a / 500.0 + fy;
        let fz = fy - b / 200.0;

        Color::new(f(fx) * WHITE.0, f(fy) * WHITE.1, f(fz) * WHITE.2)
    }
}

impl From<CieXyz<D50>> for Color<CieLab<D50>> {
    fn from(xyz: Color<CieXyz<D50>>) -> Self {
        let (x, y, z) = xyz.tuple();

        fn f(value: crate::Float) -> crate::Float {
            if value > crate::spaces::xyz_d50::E {
                value.cbrt()
            } else {
                (crate::spaces::xyz_d50::K * value + 16.0) / 116.0
            }
        }

        let f0 = f(x / WHITE.0);
        let f1 = f(y / WHITE.1);
        let f2 = f(z / WHITE.2);

        Color::new(116.0 * f1 - 16.0, 500.0 * (f0 - f1), 200.0 * (f1 - f2))
    }
}

impl From<CieLab<D50>> for Color<Srgb> {
    fn from(lab: Color<CieLab<D50>>) -> Self {
        lab.into::<CieXyz<D50>>().into()
    }
}

impl From<Srgb> for Color<CieLab<D50>> {
    fn from(rgb: Color<Srgb>) -> Self {
        let xyz = rgb.into::<CieXyz<D50>>();
        let (l, mut a, mut b) = xyz.into::<CieLab<D50>>().tuple();

        // Fixes achromatic RGB colors having a _slight_ chroma due to floating-point errors
        // and approximated computations in sRGB <-> CIELab.
        // See: https://github.com/d3/d3-color/pull/46
        let (red, green, blue) = rgb.tuple();
        if red == green && green == blue {
            a = 0.0;
            b = 0.0;
        }
        Color::new(l, a, b)
    }
}
