use crate::illuminate::{D65, D65_WHITE as WHITE};
use crate::spaces::{CieLab, CieXyz, Srgb};
use crate::{Color, Into};

impl Into<CieXyz<D65>> for Color<CieLab<D65>> {
    fn into(self, _: CieXyz<D65>) -> Color<CieXyz<D65>> {
        let (l, a, b) = self.tuple();

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

        Color::of(f(fx) * WHITE.0, f(fy) * WHITE.1, f(fz) * WHITE.2)
    }
}

impl Into<CieLab<D65>> for Color<CieXyz<D65>> {
    fn into(self, _: CieLab<D65>) -> Color<CieLab<D65>> {
        let (x, y, z) = self.tuple();

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

        Color::of(116.0 * f1 - 16.0, 500.0 * (f0 - f1), 200.0 * (f1 - f2))
    }
}

impl Into<Srgb> for Color<CieLab<D65>> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(CieXyz(D65)).into(s)
    }
}

impl Into<CieLab<D65>> for Color<Srgb> {
    fn into(self, _: CieLab<D65>) -> Color<CieLab<D65>> {
        let mut lab = self.into(CieXyz(D65)).into(CieLab(D65));

        // Fixes achromatic RGB colors having a _slight_ chroma due to floating-point errors
        // and approximated computations in sRGB <-> CIELab.
        // See: https://github.com/d3/d3-color/pull/46
        let (red, green, blue) = self.tuple();
        if red == green && green == blue {
            *lab.mut_1() = 0.0;
            *lab.mut_2() = 0.0;
        }

        lab
    }
}

#[cfg(test)]
mod tests {
    use crate::illuminate::D65;
    use crate::spaces::CieLab;
    use crate::test_util::round_trips_srgb;

    #[test]
    fn test_cielab_d65_roundtrips() {
        round_trips_srgb::<CieLab<D65>>();
    }
}
