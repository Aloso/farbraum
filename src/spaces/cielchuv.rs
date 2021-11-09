use crate::illuminate::D50;
use crate::spaces::{util, CieLchuv, CieLuv, CieXyz, Srgb};
use crate::{Color, From};

impl From<CieLchuv> for Color<CieLuv> {
    fn from(lchuv: Color<CieLchuv>) -> Self {
        let (l, c, h) = lchuv.tuple();
        let (u, v) = util::cos_and_sin_radians(c, h);

        Color::new(l, u, v)
    }
}

impl From<CieLuv> for Color<CieLchuv> {
    fn from(luv: Color<CieLuv>) -> Self {
        let (l, u, v) = luv.tuple();
        let c = (u * u + v * v).sqrt();

        let h = if c > 0.0 {
            util::normalize_hue((v.atan2(u) * 180.0) / float!(PI))
        } else {
            0.0
        };
        Color::new(l, c, h)
    }
}

impl From<Srgb> for Color<CieLchuv> {
    fn from(rgb: Color<Srgb>) -> Self {
        rgb.into::<CieXyz<D50>>().into::<CieLuv>().into()
    }
}

impl From<CieLchuv> for Color<Srgb> {
    fn from(lchuv: Color<CieLchuv>) -> Self {
        lchuv.into::<CieLuv>().into::<CieXyz<D50>>().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::spaces::{CieLchuv, CieLuv};
    use crate::test_util::{round_trips, round_trips_srgb};

    #[test]
    fn test_cielchuv_roundtrips() {
        round_trips_srgb::<CieLchuv>();
        round_trips::<CieLchuv, CieLuv>();
        round_trips::<CieLuv, CieLchuv>();
    }
}
