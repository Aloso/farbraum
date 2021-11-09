use crate::spaces::{lab, LinearSrgb, OkLab, OkLch, Srgb};
use crate::{Color, From};

impl From<OkLab> for Color<OkLch> {
    fn from(lab: Color<OkLab>) -> Self {
        lab::lab_to_lch(lab)
    }
}

impl From<OkLch> for Color<OkLab> {
    fn from(lch: Color<OkLch>) -> Self {
        lab::lch_to_lab(lch)
    }
}

impl From<OkLch> for Color<LinearSrgb> {
    fn from(lch: Color<OkLch>) -> Self {
        lch.into::<OkLab>().into()
    }
}

impl From<LinearSrgb> for Color<OkLch> {
    fn from(rgb: Color<LinearSrgb>) -> Self {
        rgb.into::<OkLab>().into()
    }
}

impl From<OkLch> for Color<Srgb> {
    fn from(lch: Color<OkLch>) -> Self {
        lch.into::<OkLab>().into::<LinearSrgb>().into()
    }
}

impl From<Srgb> for Color<OkLch> {
    fn from(rgb: Color<Srgb>) -> Self {
        rgb.into::<LinearSrgb>().into::<OkLab>().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::spaces::{OkLab, OkLch};
    use crate::test_util::{round_trips, round_trips_srgb};

    #[test]
    fn test_oklch_roundtrips() {
        round_trips_srgb::<OkLch>();
        round_trips::<OkLab, OkLch>();
        round_trips::<OkLch, OkLab>();
    }
}
