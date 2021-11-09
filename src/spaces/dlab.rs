use crate::illuminate::D65;
use crate::spaces::{CieLab, DLab, DLch, Srgb};
use crate::{Color, From};

impl From<DLab> for Color<CieLab<D65>> {
    fn from(lab: Color<DLab>) -> Self {
        lab.into::<DLch>().into()
    }
}

impl From<CieLab<D65>> for Color<DLab> {
    fn from(lab: Color<CieLab<D65>>) -> Self {
        lab.into::<DLch>().into()
    }
}

impl From<Srgb> for Color<DLab> {
    fn from(rgb: Color<Srgb>) -> Self {
        rgb.into::<CieLab<D65>>().into()
    }
}

impl From<DLab> for Color<Srgb> {
    fn from(lab: Color<DLab>) -> Self {
        lab.into::<CieLab<D65>>().into()
    }
}

#[cfg(test)]
mod tests {

    use crate::spaces::{DLab, Srgb};
    use crate::test_util::round_trips_srgb;
    use crate::{Color, Float};

    fn rgb(r: Float, g: Float, b: Float) -> Color<Srgb> {
        Color::new(r, g, b)
    }

    fn dlab(l: Float, a: Float, b: Float) -> Color<DLab> {
        Color::new(l, a, b)
    }

    #[test]
    fn test_dlab() {
        assert_eq!(
            rgb(1.0, 1.0, 1.0).into(),
            dlab(100.00000042980086, 0.0, 0.0)
        );
        let x11 = 1.0 / 15.0;
        assert_eq!(rgb(x11, x11, x11).into(), dlab(5.938147698096487, 0.0, 0.0));
        assert_eq!(rgb(0.0, 0.0, 0.0).into(), dlab(0.0, 0.0, 0.0));
        assert_eq!(
            rgb(1.0, 0.0, 0.0).into(),
            dlab(57.29278122742003, 39.49866237448939, 30.518156672666002)
        );
    }

    #[test]
    fn test_dlab_roundtrips() {
        round_trips_srgb::<DLab>();
    }
}
