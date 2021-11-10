use crate::spaces::{lab, LinearSrgb, OkLab, OkLch, Srgb};
use crate::{Color, Into};

impl Into<OkLab> for Color<OkLch> {
    fn into(self, s: OkLab) -> Color<OkLab> {
        lab::lch_to_lab(self, s)
    }
}

impl Into<OkLch> for Color<OkLab> {
    fn into(self, s: OkLch) -> Color<OkLch> {
        lab::lab_to_lch(self, s)
    }
}

impl Into<LinearSrgb> for Color<OkLch> {
    fn into(self, s: LinearSrgb) -> Color<LinearSrgb> {
        self.into(OkLab).into(s)
    }
}

impl Into<OkLch> for Color<LinearSrgb> {
    fn into(self, s: OkLch) -> Color<OkLch> {
        self.into(OkLab).into(s)
    }
}

impl Into<Srgb> for Color<OkLch> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(OkLab).into(LinearSrgb).into(s)
    }
}

impl Into<OkLch> for Color<Srgb> {
    fn into(self, s: OkLch) -> Color<OkLch> {
        self.into(LinearSrgb).into(OkLab).into(s)
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
