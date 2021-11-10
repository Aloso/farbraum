use crate::illuminate::{D50, D65};
use crate::spaces::{lab, CieLab, CieLch, Srgb};
use crate::{Color, Into};

impl<I> Into<CieLch<I>> for Color<CieLab<I>> {
    fn into(self, s: CieLch<I>) -> Color<CieLch<I>> {
        lab::lab_to_lch(self, s)
    }
}

impl<I> Into<CieLab<I>> for Color<CieLch<I>> {
    fn into(self, s: CieLab<I>) -> Color<CieLab<I>> {
        lab::lch_to_lab(self, s)
    }
}

impl Into<Srgb> for Color<CieLch<D50>> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(CieLab(D50)).into(s)
    }
}

impl Into<Srgb> for Color<CieLch<D65>> {
    fn into(self, s: Srgb) -> Color<Srgb> {
        self.into(CieLab(D65)).into(s)
    }
}

impl Into<CieLch<D50>> for Color<Srgb> {
    fn into(self, s: CieLch<D50>) -> Color<CieLch<D50>> {
        self.into(CieLab(D50)).into(s)
    }
}

impl Into<CieLch<D65>> for Color<Srgb> {
    fn into(self, s: CieLch<D65>) -> Color<CieLch<D65>> {
        self.into(CieLab(D65)).into(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::illuminate::{D50, D65};
    use crate::spaces::{CieLab, CieLch};
    use crate::test_util::{round_trips, round_trips_srgb};

    #[test]
    fn test_cielch_roundtrips() {
        round_trips_srgb::<CieLch<D50>>();
        round_trips_srgb::<CieLch<D65>>();
        round_trips::<CieLch<D65>, CieLab<D65>>();
        round_trips::<CieLab<D65>, CieLch<D65>>();
    }
}
