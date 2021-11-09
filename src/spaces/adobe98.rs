use crate::illuminate::D65;
use crate::spaces::{Adobe98, CieXyz, Srgb};
use crate::{Color, Float, Into};

fn linearize(v: Float) -> Float {
    v.abs().powf(563.0 / 256.0) * v.signum()
}

impl Into<CieXyz<D65>> for Color<Adobe98> {
    fn into(self, _: CieXyz<D65>) -> Color<CieXyz<D65>> {
        let (r, g, b) = self.tuple();
        let r = linearize(r);
        let g = linearize(g);
        let b = linearize(b);
        Color::of(
            0.5766690429101305 * r + 0.1855582379065463 * g + 0.1882286462349947 * b,
            0.29734497525053605 * r + 0.6273635662554661 * g + 0.07529145849399788 * b,
            0.02703136138641234 * r + 0.07068885253582723 * g + 0.9913375368376388 * b,
        )
    }
}

fn gamma(v: Float) -> Float {
    v.abs().powf(256.0 / 563.0) * v.signum()
}

impl Into<Adobe98> for Color<CieXyz<D65>> {
    fn into(self, _: Adobe98) -> Color<Adobe98> {
        let (x, y, z) = self.tuple();
        Color::of(
            gamma(x * 2.0415879038107465 - y * 0.5650069742788596 - 0.34473135077832956 * z),
            gamma(x * -0.9692436362808795 + y * 1.8759675015077202 + 0.04155505740717557 * z),
            gamma(x * 0.013444280632031142 - y * 0.11836239223101838 + 1.0151749943912054 * z),
        )
    }
}

impl Into<Adobe98> for Color<Srgb> {
    fn into(self, s: Adobe98) -> Color<Adobe98> {
        self.into(CieXyz(D65)).into(s)
    }
}

impl Into<Srgb> for Color<Adobe98> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(CieXyz(D65)).into(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::spaces::{Adobe98, Srgb};
    use crate::test_util::round_trips_srgb;
    use crate::{Color, Float};

    fn rgb(r: Float, g: Float, b: Float) -> Color<Srgb> {
        Color::of(r, g, b)
    }

    fn a98(r: Float, g: Float, b: Float) -> Color<Adobe98> {
        Color::of(r, g, b)
    }

    #[test]
    fn test_adobe98() {
        assert_similar!(
            rgb(1.0, 1.0, 1.0).into(Adobe98),
            a98(1.0000487485, 0.9999895104, 0.99989495005)
        );
        assert_similar!(rgb(0.0, 0.0, 0.0).into(Adobe98), a98(0.0, 0.0, 0.0));
        assert_similar!(
            rgb(1.0, 0.0, 0.0).into(Adobe98),
            a98(0.85865373273, -0.000124357058167, 0.000221323599465)
        );
    }

    #[test]
    fn test_adobe98_roundtrips() {
        round_trips_srgb::<Adobe98>();
    }
}
