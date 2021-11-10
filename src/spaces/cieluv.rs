use crate::illuminate::{D50, D50_WHITE};
use crate::spaces::{util, CieLuv, CieXyz, Srgb};
use crate::{Color, Float, Into};

use super::xyz::{E, K};

fn u_fn(x: Float, y: Float, z: Float) -> Float {
    (4.0 * x) / (x + 15.0 * y + 3.0 * z)
}
fn v_fn(x: Float, y: Float, z: Float) -> Float {
    (9.0 * y) / (x + 15.0 * y + 3.0 * z)
}

impl Into<CieXyz<D50>> for Color<CieLuv> {
    fn into(self, _: CieXyz<D50>) -> Color<CieXyz<D50>> {
        let (l, u, v) = self.tuple();

        let (xn, yn, zn) = D50_WHITE;

        let un = u_fn(xn, yn, zn);
        let vn = v_fn(xn, yn, zn);

        let up = util::no_nan(u / (13.0 * l)) + un;
        let vp = util::no_nan(v / (13.0 * l)) + vn;

        let y = yn
            * (if l <= 8.0 {
                l / K
            } else {
                ((l + 16.0) / 116.0).powi(3)
            });
        let x = (y * (9.0 * up)) / (4.0 * vp);
        let z = (y * (12.0 - 3.0 * up - 20.0 * vp)) / (4.0 * vp);

        Color::of(x, y, z)
    }
}

fn l_fn(value: Float) -> Float {
    if value <= E {
        K * value
    } else {
        116.0 * value.cbrt() - 16.0
    }
}

impl Into<CieLuv> for Color<CieXyz<D50>> {
    fn into(self, _: CieLuv) -> Color<CieLuv> {
        let (x, y, z) = self.tuple();

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

        Color::of(l, u, v)
    }
}

impl Into<CieLuv> for Color<Srgb> {
    fn into(self, s: CieLuv) -> Color<CieLuv> {
        self.into(CieXyz(D50)).into(s)
    }
}

impl Into<Srgb> for Color<CieLuv> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(CieXyz(D50)).into(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::illuminate::D50;
    use crate::spaces::{CieLuv, CieXyz};
    use crate::test_util::{round_trips, round_trips_srgb};

    #[test]
    fn test_cieluv_roundtrips() {
        round_trips_srgb::<CieLuv>();
        round_trips::<CieXyz<D50>, CieLuv>();
        round_trips::<CieLuv, CieXyz<D50>>();
    }
}
