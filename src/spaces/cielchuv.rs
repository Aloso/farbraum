use crate::illuminate::D50;
use crate::spaces::{util, CieLchuv, CieLuv, CieXyz, Srgb};
use crate::{Color, Into};

impl Into<CieLuv> for Color<CieLchuv> {
    fn into(self, _: CieLuv) -> Color<CieLuv> {
        let (l, c, h) = self.tuple();
        let (u, v) = util::cos_and_sin_radians(c, h);

        Color::of(l, u, v)
    }
}

impl Into<CieLchuv> for Color<CieLuv> {
    fn into(self, _: CieLchuv) -> Color<CieLchuv> {
        let (l, u, v) = self.tuple();
        let c = (u * u + v * v).sqrt();

        let h = if c > 0.0 {
            util::normalize_hue((v.atan2(u) * 180.0) / float!(PI))
        } else {
            0.0
        };
        Color::of(l, c, h)
    }
}

impl Into<CieLchuv> for Color<Srgb> {
    fn into(self, s: CieLchuv) -> Color<CieLchuv> {
        self.into(CieXyz(D50)).into(CieLuv).into(s)
    }
}

impl Into<Srgb> for Color<CieLchuv> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(CieLuv).into(CieXyz(D50)).into(s)
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
