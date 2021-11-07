use crate::spaces::{CieLab, CieLch};
use crate::{lab_util, From, Color};

impl<WHITE> From<CieLch<WHITE>> for Color<CieLab<WHITE>> {
    fn from(lch: Color<CieLch<WHITE>>) -> Self {
        lab_util::lch_to_lab(lch)
    }
}

impl<WHITE> From<CieLab<WHITE>> for Color<CieLch<WHITE>> {
    fn from(lab: Color<CieLab<WHITE>>) -> Self {
        lab_util::lab_to_lch(lab)
    }
}
