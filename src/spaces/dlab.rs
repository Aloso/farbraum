use crate::spaces::{DLab, DLch};
use crate::{lab_util, From, Color};

impl From<DLch> for Color<DLab> {
    fn from(lch: Color<DLch>) -> Self {
        lab_util::lch_to_lab(lch)
    }
}

impl From<DLab> for Color<DLch> {
    fn from(lab: Color<DLab>) -> Self {
        lab_util::lab_to_lch(lab)
    }
}
