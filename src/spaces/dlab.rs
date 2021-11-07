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
