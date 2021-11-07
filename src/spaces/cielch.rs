use crate::illuminate::{D50, D65};
use crate::spaces::{lab, CieLab, CieLch, Srgb};
use crate::{Color, From};

impl<I> From<CieLch<I>> for Color<CieLab<I>> {
    fn from(lch: Color<CieLch<I>>) -> Self {
        lab::lch_to_lab(lch)
    }
}

impl<I> From<CieLab<I>> for Color<CieLch<I>> {
    fn from(lab: Color<CieLab<I>>) -> Self {
        lab::lab_to_lch(lab)
    }
}

impl From<CieLch<D50>> for Color<Srgb> {
    fn from(lch: Color<CieLch<D50>>) -> Self {
        lch.into::<CieLab<D50>>().into()
    }
}

impl From<CieLch<D65>> for Color<Srgb> {
    fn from(lch: Color<CieLch<D65>>) -> Self {
        lch.into::<CieLab<D65>>().into()
    }
}

impl From<Srgb> for Color<CieLch<D50>> {
    fn from(rgb: Color<Srgb>) -> Self {
        rgb.into::<CieLab<D50>>().into()
    }
}

impl From<Srgb> for Color<CieLch<D65>> {
    fn from(rgb: Color<Srgb>) -> Self {
        rgb.into::<CieLab<D65>>().into()
    }
}
