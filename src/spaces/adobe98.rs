use crate::illuminate::D65;
use crate::spaces::{Adobe98, CieXyz, Srgb};
use crate::{Color, Float, From};

fn linearize(v: Float) -> Float {
    v.abs().powf(563.0 / 256.0) * v.signum()
}

impl From<Adobe98> for Color<CieXyz<D65>> {
    fn from(a_rgb: Color<Adobe98>) -> Self {
        let (r, g, b) = a_rgb.tuple();
        let r = linearize(r);
        let g = linearize(g);
        let b = linearize(b);
        Color::new(
            0.5766690429101305 * r + 0.1855582379065463 * g + 0.1882286462349947 * b,
            0.29734497525053605 * r + 0.6273635662554661 * g + 0.07529145849399788 * b,
            0.02703136138641234 * r + 0.07068885253582723 * g + 0.9913375368376388 * b,
        )
    }
}

fn gamma(v: Float) -> Float {
    v.abs().powf(256.0 / 563.0) * v.signum()
}

impl From<CieXyz<D65>> for Color<Adobe98> {
    fn from(xyz: Color<CieXyz<D65>>) -> Self {
        let (x, y, z) = xyz.tuple();
        Color::new(
            gamma(x * 2.0415879038107465 - y * 0.5650069742788596 - 0.34473135077832956 * z),
            gamma(x * -0.9692436362808795 + y * 1.8759675015077202 + 0.04155505740717557 * z),
            gamma(x * 0.013444280632031142 - y * 0.11836239223101838 + 1.0151749943912054 * z),
        )
    }
}

impl From<Adobe98> for Color<Srgb> {
    fn from(rgb: Color<Adobe98>) -> Self {
        rgb.into::<CieXyz<D65>>().into()
    }
}

impl From<Srgb> for Color<Adobe98> {
    fn from(rgb: Color<Srgb>) -> Self {
        rgb.into::<CieXyz<D65>>().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::spaces::{Adobe98, Srgb};
    use crate::test_util::round_trips_srgb;
    use crate::{Color, Float};

    fn rgb(r: Float, g: Float, b: Float) -> Color<Srgb> {
        Color::new(r, g, b)
    }

    fn a98(r: Float, g: Float, b: Float) -> Color<Adobe98> {
        Color::new(r, g, b)
    }

    #[test]
    fn test_adobe98() {
        assert_similar!(
            rgb(1.0, 1.0, 1.0).into(),
            a98(1.0000487485, 0.9999895104, 0.99989495005)
        );
        assert_similar!(rgb(0.0, 0.0, 0.0).into(), a98(0.0, 0.0, 0.0));
        assert_similar!(
            rgb(1.0, 0.0, 0.0).into(),
            a98(0.85865373273, -0.000124357058167, 0.000221323599465)
        );
    }

    #[test]
    fn test_adobe98_roundtrips() {
        round_trips_srgb::<Adobe98>();
    }
}
