use crate::spaces::{CieLab, CieLch};
use crate::{lab_util, From, Vec3};

impl<WHITE> From<CieLch, WHITE> for Vec3<CieLab, WHITE> {
    fn from(lch: Vec3<CieLch, WHITE>) -> Self {
        lab_util::lch_to_lab(lch)
    }
}

impl<WHITE> From<CieLab, WHITE> for Vec3<CieLch, WHITE> {
    fn from(lab: Vec3<CieLab, WHITE>) -> Self {
        lab_util::lab_to_lch(lab)
    }
}
