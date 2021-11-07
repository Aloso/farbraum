use crate::illuminate::{D50, D50_WHITE};
use crate::spaces::{CieLuv, CieXyz, Srgb};
use crate::{Color, Float, From};

use super::xyz_d50::{E, K};

fn u_fn(x: Float, y: Float, z: Float) -> Float {
    (4.0 * x) / (x + 15.0 * y + 3.0 * z)
}
fn v_fn(x: Float, y: Float, z: Float) -> Float {
    (9.0 * y) / (x + 15.0 * y + 3.0 * z)
}

impl From<CieLuv> for Color<CieXyz<D50>> {
    fn from(luv: Color<CieLuv>) -> Self {
        let (l, u, v) = luv.tuple();

        let (xn, yn, zn) = D50_WHITE;

        let un = u_fn(xn, yn, zn);
        let vn = v_fn(xn, yn, zn);

        let up = u / (13.0 * l) + un;
        let vp = v / (13.0 * l) + vn;

        let y = yn
            * (if l <= 8.0 {
                l / K
            } else {
                ((l + 16.0) / 116.0).powi(3)
            });
        let x = (y * (9.0 * up)) / (4.0 * vp);
        let z = (y * (12.0 - 3.0 * up - 20.0 * vp)) / (4.0 * vp);

        Color::new(x, y, z)
    }
}

fn l_fn(value: Float) -> Float {
    if value <= E {
        K * value
    } else {
        116.0 * value.cbrt() - 16.0
    }
}

impl From<CieXyz<D50>> for Color<CieLuv> {
    fn from(xyz: Color<CieXyz<D50>>) -> Self {
        let (x, y, z) = xyz.tuple();

        let (xn, yn, zn) = D50_WHITE;

        let mut l = l_fn(y / yn);
        let mut u = u_fn(x, y, z);
        let mut v = v_fn(x, y, z);

        // guard against NaNs produced by `xyz(0 0 0)` black
        if !u.is_finite() || !v.is_finite() {
            l = 0.0;
            u = 0.0;
            v = 0.0;
        } else {
            let un = u_fn(xn, yn, zn);
            let vn = v_fn(xn, yn, zn);

            u = 13.0 * l * (u - un);
            v = 13.0 * l * (v - vn);
        }

        Color::new(l, u, v)
    }
}

impl From<Srgb> for Color<CieLuv> {
    fn from(rgb: Color<Srgb>) -> Self {
        rgb.into::<CieXyz<D50>>().into()
    }
}

impl From<CieLuv> for Color<Srgb> {
    fn from(luv: Color<CieLuv>) -> Self {
        luv.into::<CieXyz<D50>>().into()
    }
}
