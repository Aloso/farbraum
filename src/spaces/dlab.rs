use crate::spaces::{DLab, DLch};
use crate::{lab_util, From, Vec3};

impl<WHITE> From<DLch, WHITE> for Vec3<DLab, WHITE> {
    fn from(lch: Vec3<DLch, WHITE>) -> Self {
        lab_util::lch_to_lab(lch)
    }
}

impl<WHITE> From<DLab, WHITE> for Vec3<DLch, WHITE> {
    fn from(lab: Vec3<DLab, WHITE>) -> Self {
        lab_util::lab_to_lch(lab)
    }
}
