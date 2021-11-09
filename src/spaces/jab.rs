use crate::illuminate::D65;
use crate::spaces::{CieXyz, Jab, Srgb};
use crate::{Color, Float, From};

const N: Float = 0.1593017578125; // = 2610 / Math.pow(2, 14);
const P: Float = 134.034375; // = 1.7 * 2523 / Math.pow(2, 5);
const C1: Float = 0.8359375; // = 3424 / Math.pow(2, 12);
const C2: Float = 18.8515625; // = 2413 / Math.pow(2, 7);
const C3: Float = 18.6875; // = 2392 / Math.pow(2, 7);
const D0: Float = 1.6295499532821566e-11;

// `v` may be negative, in which case return 0 instead of NaN
fn pq_inv(v: Float) -> Float {
    let vp = v.powf(1.0 / P);
    let pow = ((C1 - vp) / (C3 * vp - C2)).powf(1.0 / N);
    if pow.is_nan() {
        0.0
    } else {
        10_000.0 * pow
    }
}

fn rel(v: Float) -> Float {
    v / 203.0
}

impl From<Jab> for Color<CieXyz<D65>> {
    fn from(jab: Color<Jab>) -> Self {
        let (j, a, b) = jab.tuple();

        let i = (j + D0) / (0.44 + 0.56 * (j + D0));

        let l = pq_inv(i + 0.1386050432715393 * a + 0.05804731615611869 * b);
        let m = pq_inv(i - 0.1386050432715393 * a - 0.05804731615611891 * b);
        let s = pq_inv(i - 0.09601924202631895 * a - 0.811891896056039 * b);

        let x = rel(1.661373055774069 * l - 0.9145230923250668 * m + 0.2313620767186147 * s);
        let y = rel(-0.3250758740427037 * l + 1.571847038366936 * m - 0.218253831867294 * s);
        let z = rel(-0.09098281098284756 * l - 0.312728290523074 * m + 1.52276656130526 * s);

        Color::new(x, y, z)
    }
}

fn pq(v: Float) -> Float {
    let vn = (v / 10000.0).powf(N);
    let pow = ((C1 + C2 * vn) / (1.0 + C3 * vn)).powf(P);
    if pow.is_nan() {
        0.0
    } else {
        pow
    }
}

// Convert to Absolute XYZ
fn abs(v: Float) -> Float {
    (v * 203.0).max(0.0)
}

impl From<CieXyz<D65>> for Color<Jab> {
    fn from(xyz: Color<CieXyz<D65>>) -> Self {
        let (x, y, z) = xyz.tuple();

        let x = abs(x);
        let y = abs(y);
        let z = abs(z);

        let xp = 1.15 * x - 0.15 * z;
        let yp = 0.66 * y + 0.34 * x;

        let l = pq(0.41478972 * xp + 0.579999 * yp + 0.014648 * z);
        let m = pq(-0.20151 * xp + 1.120649 * yp + 0.0531008 * z);
        let s = pq(-0.0166008 * xp + 0.2648 * yp + 0.6684799 * z);

        let i = (l + m) / 2.0;

        let j = (0.44 * i) / (1.0 - 0.56 * i) - D0;
        let a = 3.524 * l - 4.066708 * m + 0.542708 * s;
        let b = 0.199076 * l + 1.096799 * m - 1.295875 * s;

        Color::new(j, a, b)
    }
}

impl From<Srgb> for Color<Jab> {
    fn from(rgb: Color<Srgb>) -> Self {
        rgb.into::<CieXyz<D65>>().into()
    }
}

impl From<Jab> for Color<Srgb> {
    fn from(jab: Color<Jab>) -> Self {
        jab.into::<CieXyz<D65>>().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::illuminate::D65;
    use crate::spaces::{CieXyz, Jab, Srgb};
    use crate::test_util::{round_trips, round_trips_srgb};
    use crate::{Color, Float};

    fn rgb(r: Float, g: Float, b: Float) -> Color<Srgb> {
        Color::new(r, g, b)
    }

    fn jab(l: Float, a: Float, b: Float) -> Color<Jab> {
        Color::new(l, a, b)
    }

    #[test]
    fn test_jab_to_rgb() {
        assert_similar!(
            jab(0.22206540515, -0.000153429, -0.000094388).into(),
            rgb(1.0, 1.0, 1.0)
        );
        assert_similar!(
            jab(3.2311742677852644e-27, 0.0, 0.0).into(),
            rgb(0.0, 0.0, 0.0)
        );
        assert_similar!(
            jab(0.13439374892, 0.11788937861, 0.111882888348).into(),
            rgb(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn test_jab_roundtrips() {
        round_trips_srgb::<Jab>();
        round_trips::<Jab, CieXyz<D65>>();
        round_trips::<CieXyz<D65>, Jab>();
    }
}
