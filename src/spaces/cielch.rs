use crate::spaces::{CieLab, CieLch};
use crate::{lab_util, Color, From};

impl<I> From<CieLch<I>> for Color<CieLab<I>> {
    fn from(lch: Color<CieLch<I>>) -> Self {
        lab_util::lch_to_lab(lch)
    }
}

impl<I> From<CieLab<I>> for Color<CieLch<I>> {
    fn from(lab: Color<CieLab<I>>) -> Self {
        lab_util::lab_to_lch(lab)
    }
}
