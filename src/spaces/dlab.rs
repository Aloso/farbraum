use crate::illuminate::D65;
use crate::spaces::{CieLab, DLab, DLch, Srgb};
use crate::{Color, Into};

impl Into<CieLab<D65>> for Color<DLab> {
    fn into(self, s: CieLab<D65>) -> Color<CieLab<D65>> {
        self.into(DLch).into(s)
    }
}

impl Into<DLab> for Color<CieLab<D65>> {
    fn into(self, s: DLab) -> Color<DLab> {
        self.into(DLch).into(s)
    }
}

impl Into<DLab> for Color<Srgb> {
    fn into(self, s: DLab) -> Color<DLab> {
        self.into(CieLab(D65)).into(s)
    }
}

impl Into<Srgb> for Color<DLab> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(CieLab(D65)).into(s)
    }
}

#[cfg(test)]
mod tests {

    use crate::spaces::{DLab, Srgb};
    use crate::test_util::round_trips_srgb;
    use crate::{Color, Float};

    fn rgb(r: Float, g: Float, b: Float) -> Color<Srgb> {
        Color::of(r, g, b)
    }

    fn dlab(l: Float, a: Float, b: Float) -> Color<DLab> {
        Color::of(l, a, b)
    }

    #[test]
    fn test_dlab() {
        assert_eq!(
            rgb(1.0, 1.0, 1.0).into(DLab),
            dlab(100.00000042980086, 0.0, 0.0)
        );
        let x11 = 1.0 / 15.0;
        assert_eq!(
            rgb(x11, x11, x11).into(DLab),
            dlab(5.938147698096487, 0.0, 0.0)
        );
        assert_eq!(rgb(0.0, 0.0, 0.0).into(DLab), dlab(0.0, 0.0, 0.0));
        assert_eq!(
            rgb(1.0, 0.0, 0.0).into(DLab),
            dlab(57.29278122742003, 39.49866237448939, 30.518156672666002)
        );
    }

    #[test]
    fn test_dlab_roundtrips() {
        round_trips_srgb::<DLab>();
    }
}
